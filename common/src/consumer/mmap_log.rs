use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::Path;
use memmap2::MmapMut;
use regex::Regex;
use serde_json::{Map, Value};

use crate::consumer::Consumer;
use crate::event::BoxedEvent;
use crate::log_info;
use crate::util::datetime::get_hour_since_epoch;
use crate::util::error::{macros::runtime_error, Result};
use crate::util::error::macros::host_error;
use crate::util::single_process_lock::{SingleProcessLock, SingleProcessLocked};
use crate::util::system_util::{LINE_ENDING, LINE_ENDING_LENGTH};

#[derive(Debug)]
pub struct MmapLogConsumer {
    // set by user
    path: String,                   // log 所在文件夹路径
    name_prefix: String,            // log 前缀
    size: u64,                      // 文件大小
    flush_size: Option<u64>,        // flush 触发大小
    // internal preserved
    mmap: MmapMut,
    offset: usize,                  // 当前写入 mem 的位置
    file_time: u64,                 // 当前 log 文件时间（精度到小时）
    revision: u16,                  // 当前分片
    flush_offset: usize,            // 最后一次 flush 的位置
    _locked: SingleProcessLocked,   // 进程安全，只允许单进程访问（根据 path + file_prefix 区分）
}

impl MmapLogConsumer {
    fn new(
        path: Option<String>, name_prefix: String,
        file_size: u64, flush_size: Option<u64>,
    ) -> Result<Self> {
        let file_time = get_hour_since_epoch();
        let path = path.unwrap_or(String::from("."));
        create_dir_all(path.clone()).unwrap();

        let lock = SingleProcessLock::new(path.clone(), name_prefix.clone())?;
        let locked = lock.lock()?;

        let mut revision: u16 = Self::get_init_revision(&path, &name_prefix, &file_time);
        let mut filename = Self::get_filename(&name_prefix, file_time, revision);
        let mut mmap = Self::open_or_create_file(&path, filename, file_size);
        let mut offset = Self::calc_offset(&mmap);
        while offset.is_none() {
            // case of file is full already.
            revision += 1;
            filename = Self::get_filename(&name_prefix, file_time, revision);
            mmap = Self::open_or_create_file(&path, filename, file_size);
            offset = Self::calc_offset(&mmap);
        }
        let offset = offset.unwrap();

        Ok(MmapLogConsumer {
            mmap,
            offset,
            size: file_size,
            flush_size,
            file_time,
            revision,
            name_prefix,
            path,
            flush_offset: offset,
            _locked: locked,
        })
    }

    pub fn from_config(config: &mut Map<String, Value>) -> Result<Box<dyn Consumer>> {
        let Some(Value::String(path)) = config.remove("path") else {
            return host_error!("Failed to initialize: missing \"path\"!");
        };

        let name_prefix: String = if let Some(Value::String(name_prefix)) = config.remove("name_prefix") {
            name_prefix
        } else {
            String::from("dt")
        };

        static DEFAULT_FILE_SIZE: u64 = 16 * 1024 * 128;            // 2 MB
        let file_size: u64 = if let Some(Value::Number(num)) = config.remove("file_size") {
            if let Some(file_size) = num.as_u64() {
                // Must be greater than 0.
                if file_size == 0 {
                    DEFAULT_FILE_SIZE
                } else {
                    file_size
                }
            } else {
                DEFAULT_FILE_SIZE
            }
        } else {
            DEFAULT_FILE_SIZE
        };

        let flush_size: Option<u64> = if let Some(Value::Number(num)) = config.remove("flush_size") {
            num.as_u64().and_then(|it| {
                // Must be greater than 0.
                if it == 0 {
                    None
                } else {
                    Some(it)
                }
            })
        } else {
            None
        };

        let consumer = MmapLogConsumer::new(
            Some(path), name_prefix, file_size, flush_size
        )?;
        Ok(Box::new(consumer))
    }

    fn flush(&mut self, is_async: bool) -> Result<()> {
        #[cfg(feature = "benchmark")]
        let st = std::time::Instant::now();
        let length = self.offset - self.flush_offset;
        if length <= 0 {
            return Ok(());
        }

        let flush_result = if is_async {
            self.mmap.flush_async_range(self.flush_offset, length)
        } else {
            self.mmap.flush_range(self.flush_offset, length)
        };

        if let Err(e) = flush_result {
            return runtime_error!(
                "Failed to flushing file ({}, {}) by range ({} -> {}), {}",
                self.path,
                Self::get_filename(&self.name_prefix, self.file_time, self.revision),
                self.flush_offset,
                self.flush_offset + length,
                e
            )
        } else {
            self.flush_offset = self.offset;
            log_info!("Flushed {} bytes!", length);
        };

        #[cfg(feature = "benchmark")]
        if is_async {
            (&crate::util::benchmark_tracer::BM_TRACER).add("Time used to flush (async)", st.elapsed().as_micros());
        } else {
            (&crate::util::benchmark_tracer::BM_TRACER).add("Time used to flush (sync)", st.elapsed().as_micros());
        }

        Ok(())
    }

    fn flush_rest_and_truncate(&mut self, is_async: bool) -> Result<()> {
        self.flush(is_async)?;
        if self.offset != self.size as usize {
            let file = Self::get_file(&self.path, Self::get_filename(&self.name_prefix, self.file_time, self.revision));
            if let Err(e) = file.set_len(self.offset as u64) {
                return runtime_error!(
                    "Failed to truncate file ({}, {}) to its size, {}",
                    self.path,
                    Self::get_filename(&self.name_prefix, self.file_time, self.revision),
                    e
                );
            }
        }

        Ok(())
    }

    fn append(&mut self, content: String, flush: bool) -> Result<()> {
        self.ensure_time_sep()?;
        let content = content.as_bytes();
        let length = content.len();
        let extra_length = if self.offset == 0 { 0 } else { LINE_ENDING_LENGTH };
        self.ensure_can_append(length + extra_length)?;

        if self.offset != 0 {
            // appending to existing file.
            self.mmap[self.offset..self.offset+LINE_ENDING_LENGTH].copy_from_slice(LINE_ENDING.as_bytes());
            self.offset += LINE_ENDING_LENGTH;
        }

        self.mmap[self.offset..self.offset+length].copy_from_slice(content);
        self.offset += length;

        let should_flush_by_flush_size: bool = if let Some(flush_size) = self.flush_size {
            // un-flushed is over flush_size
            flush_size <= (self.offset - self.flush_offset) as u64
        } else {
            false
        };

        if flush || should_flush_by_flush_size {
            self.flush(true)?;
        }

        Ok(())
    }

    fn ensure_time_sep(&mut self) -> Result<()> {
        let crt_in_hour = get_hour_since_epoch();
        if crt_in_hour <= self.file_time {
            // Time unchanged, or time invalid.
            return Ok(())
        }
        self.update_mmap(1)?;
        Ok(())
    }

    fn ensure_can_append(&mut self, append_length: usize) -> Result<bool> {
        if append_length > self.size as usize {
            Ok(false)
        } else if append_length + self.offset > self.size as usize {
            // new file
            self.update_mmap(2)?;
            // recurrently find/create file with enough room.
            self.ensure_can_append(append_length)
        } else {
            Ok(true)
        }
    }

    /// reason:
    ///   1: time changes.
    ///   2: revision changes.
    fn update_mmap(&mut self, reason: u8) -> Result<()> {
        self.flush_rest_and_truncate(true)?;

        match reason {
            2 => {
                self.revision += 1;
            }
            _ => {
                self.file_time = get_hour_since_epoch();
                self.revision = 0;
            }
        }
        let filename = Self::get_filename(&self.name_prefix, self.file_time, self.revision);
        self.mmap = Self::open_or_create_file(&self.path, filename.clone(), self.size);
        if let Some(offset) = Self::calc_offset(&self.mmap) {
            self.offset = offset;
            self.flush_offset = offset;
        } else {
            self.update_mmap(2)?;
        }
        Ok(())
    }

    fn get_init_revision(path: &String, name_prefix: &String, file_time: &u64) -> u16 {
        let paths = if let Ok(paths) = fs::read_dir(path) {
            paths
        } else {
            return 0;
        };

        let re = format!("{}-{}_([0-9]+).log", name_prefix, file_time);
        let regex = Regex::new(re.as_str()).unwrap();

        let mut revision: u16 = 0;
        for path in paths {
            if let Ok(path) = path {
                let file_name = path.file_name();
                let name = file_name.to_str().unwrap();
                for (_, [old]) in regex.captures_iter(name).map(|c| c.extract()) {
                    if let Ok(old) = old.parse::<u16>() {
                        if revision <= old {
                            revision = old;
                        }
                    }
                }
            }
        }
        revision
    }

    fn get_filename(name_prefix: &String, file_time: u64, revision: u16) -> String {
        format!("{}-{}_{}.log", name_prefix, file_time, revision)
    }

    fn open_or_create_file(path: &String, filename: String, size: u64) -> MmapMut {
        let file = Self::get_file(path, filename);
        file.set_len(size).expect("Failed to set file length");      // 2 MB
        unsafe { MmapMut::map_mut(&file).unwrap() }
    }

    fn get_file(path: &String, filename: String) -> File {
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(Path::new(path).join(filename))
            .unwrap()
    }

    fn calc_offset(mmap: &MmapMut) -> Option<usize> {
        // match \0
        mmap.iter().position(|&it | it == 0)
    }
}

impl Consumer for MmapLogConsumer {
    fn add(self: &mut Self, event: BoxedEvent) -> Result<()> {
        if let Ok(json) = serde_json::to_string(&event) {
            self.append(json, false)
        } else {
            runtime_error!("Failed to jsonify this event: {event:?}")
        }
    }

    fn flush(self: &mut Self) -> Result<()> {
        self.flush(true)
    }

    fn close(self: &mut Self) -> Result<()> {
        self.flush_rest_and_truncate(false)
    }
}