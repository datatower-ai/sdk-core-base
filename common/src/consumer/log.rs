use std::collections::VecDeque;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use regex::Regex;
use serde_json::{Map, Value};

use crate::{log_error, log_info};
use crate::consumer::Consumer;
use crate::event::BoxedEvent;
use crate::util::datetime::get_hour_since_epoch;
use crate::util::error::macros::{host_error, runtime_error};
use crate::util::error::Result;

/**
 * Should be run in a single thread for current implementation.
 */
#[derive(Debug)]
pub struct LogConsumer {
    // Sets from outer
    path: String,
    max_batch_len: u32,             // Affects frequency of flush.
    name_prefix: Option<String>,
    max_file_size_bytes: Option<u64>,       // Affects number of files created within interval.
    // Internally reserved
    crt_size_bytes: u64,
    batch: VecDeque<String>,
    file_time: u64,
    revision: u16,                  // for multiple log file created in a single time interval
}

impl LogConsumer {
    pub fn new(
        path: String,
        max_batch_len: u32,
        name_prefix: Option<String>,
        max_file_size_bytes: Option<u64>
    ) -> Self {
        let file_time = get_hour_since_epoch();
        let (revision, crt_size_bytes) = LogConsumer::get_init_revision(&path, &name_prefix, &file_time);
        LogConsumer {
            path, max_batch_len, name_prefix, max_file_size_bytes,
            revision, file_time, crt_size_bytes,
            batch: VecDeque::new(),
        }
    }

    pub fn from_config(config: &mut Map<String, Value>) -> Result<Box<dyn Consumer>> {
        let Some(Value::String(path)) = config.remove("path") else {
            return host_error!("Failed to initialize: missing \"path\"!");
        };

        let max_batch_len = config.remove("max_batch_len");
        let Some(Value::Number(max_batch_len)) = max_batch_len else {
            return host_error!("Failed to initialize: missing \"max_batch_len\"!");
        };
        let max_batch_len = if let Some(max_batch_len) = max_batch_len.as_u64() {
            if max_batch_len == 0 {
                return host_error!("Failed to initialize: \"max_batch_len\" cannot be ZERO!");
            }
            max_batch_len
        } else {
            return host_error!("Failed to initialize: \"max_batch_len\" should be a positive number!");
        };

        let name_prefix: Option<String> = if let Some(Value::String(name_prefix)) = config.remove("name_prefix") {
            Some(name_prefix)
        } else {
            None
        };
        let max_file_size_bytes = config.remove("max_file_size_bytes");
        let max_file_size_bytes: Option<u64> = if let Some(Value::Number(max_file_size_bytes)) = max_file_size_bytes {
            let max_file_size_bytes = max_file_size_bytes.as_u64();
            let num = max_file_size_bytes.unwrap_or(0);
            if num == 0 {
                // for the port that cannot set default/none arg value, uses 0 instead for unlimited.
                None
            } else {
                // With minimum value?
                max_file_size_bytes
            }
        } else {
            None
        };

        let consumer = LogConsumer::new(
            path, max_batch_len as u32, name_prefix, max_file_size_bytes
        );
        Ok(Box::new(consumer))
    }

    pub fn get_init_revision(path: &String, name_prefix: &Option<String>, file_time: &u64) -> (u16, u64) {
        let paths = if let Ok(paths) = fs::read_dir(path) {
            paths
        } else {
            return (0, 0);
        };

        let default_name_prefix = &String::from("dt");
        let name_prefix = if let Some(name_prefix) = name_prefix {
            name_prefix
        } else {
            default_name_prefix
        };
        let re = format!("{}-{}_([0-9]+).log", name_prefix, file_time);
        let regex = Regex::new(re.as_str()).unwrap();

        let mut revision: u16 = 0;
        let mut file_size_byte: u64 = 0;
        for path in paths {
            if let Ok(path) = path {
                let file_name = path.file_name();
                let name = file_name.to_str().unwrap();
                for (_, [old]) in regex.captures_iter(name).map(|c| c.extract()) {
                    if let Ok(old) = old.parse::<u16>() {
                        if revision <= old {
                            revision = old;
                            file_size_byte = path.metadata().unwrap().len();
                        }
                    }
                }
            }
        }
        (revision, file_size_byte)
    }

    fn is_time_changed(self: &Self) -> bool {
        let crt_in_hour = get_hour_since_epoch();
        crt_in_hour > self.file_time
    }

    fn is_need_flush(self: &Self) -> bool {
        self.batch.len() as u32 >= self.max_batch_len
    }

    fn get_filename(self: &mut Self) -> String {
        if let Some(name_prefix) =  &self.name_prefix {
            if !name_prefix.is_empty() {
                return format!("{}-{}_{}.log", name_prefix, self.file_time, self.revision);
            }
        }
        format!("dt-{}_{}.log", self.file_time, self.revision)
    }

    /// refresh_mode:
    ///     - 0: No need to refresh.
    ///     - 1: Refresh by time.
    ///     - 2: Refresh by size.
    fn write_to_file(self: &mut Self, refresh_mode: u8) {
        // Once threading support needed, wrap this with a mutex!
        if !self.batch.is_empty() {
            #[cfg(feature = "benchmark")]
            let st = std::time::Instant::now();
            let filename = self.get_filename();

            let path = Path::new(&self.path);
            let _ = fs::create_dir_all(&path);

            let file_path = path.join(filename);

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_path)
                .unwrap();

            let mut n = 0;
            while let Some(s) = self.batch.pop_front() {
                if let Err(e) = writeln!(file, "{}", s) {
                    log_error!("Couldn't write to file: {}", e);
                } else {
                    n += 1;
                }
            }
            file.sync_all().expect("File sync failed");
            self.crt_size_bytes = file.metadata().unwrap().len();
            log_info!("Flushed {} events!", n);
            #[cfg(feature = "benchmark")]
            (&crate::util::benchmark_tracer::BM_TRACER).add("Time used to flush", st.elapsed().as_micros());
        }

        // Once threading support needed, wrap this with a mutex!
        // Updating naming factors.
        match refresh_mode {
            1 => {
                self.file_time = get_hour_since_epoch();
                self.revision = 0;
                self.crt_size_bytes = 0;
            },
            2 => {
                self.revision += 1;
                self.crt_size_bytes = 0;
            },
            _ => {},
        }
    }
}

impl Consumer for LogConsumer {
    fn add(self: &mut Self, event: BoxedEvent) -> Result<()> {
        if self.is_time_changed() {
            self.write_to_file(1);
        }

        if let Ok(json) = serde_json::to_string(&event) {
            let json_size = json.len() as u64;
            if let Some(max_file_size_bytes) = self.max_file_size_bytes {
                if self.crt_size_bytes + json_size > max_file_size_bytes {
                    self.write_to_file(2);
                }
            }
            self.batch.push_back(json);
            self.crt_size_bytes += json_size;
        } else {
            return runtime_error!("Failed to jsonify this event: {event:?}");
        }

        if self.is_need_flush() {
            self.write_to_file(0);
        }
        Ok(())
    }

    fn flush(self: &mut Self) -> Result<()> {
        while !self.batch.is_empty() {
            self.write_to_file(0);
        }
        Ok(())
    }

    fn close(self: &mut Self) -> Result<()> {
        while !self.batch.is_empty() {
            self.write_to_file(0);
        }
        Ok(())
    }
}

impl Default for LogConsumer {
    fn default() -> Self {
        LogConsumer::new("./log".to_string(), 100, None, None)
    }
}

impl Drop for LogConsumer {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn its_work() {
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backward");
        let since_epoch_sec = since_epoch.as_secs();
        let since_epoch_hour = since_epoch_sec - since_epoch_sec % 3600;
        println!("Time: {}", since_epoch.as_secs());
        println!("Time in hour: {}", since_epoch_hour);
    }
}