use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use regex::Regex;
use serde_json::{Map, Value};
use crate::consumer::Consumer;
use crate::{log_error, log_info};
use crate::event::processing::process_event;
use crate::util::datetime::get_hour_since_epoch;

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
    batch: Vec<String>,
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
            batch: Vec::new(),
        }
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
            while let Some(s) = self.batch.pop() {
                if let Err(e) = writeln!(file, "{}", s) {
                    log_error!("Couldn't write to file: {}", e);
                } else {
                    n += 1;
                }
            }
            file.sync_all().expect("File sync failed");
            self.crt_size_bytes = file.metadata().unwrap().len();
            log_info!("Flushed {} events!", n)
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
    fn add(self: &mut Self, mut event: Map<String, Value>) -> bool {
        if !process_event(&mut event) {
            log_error!("Verification failed for this event: {:?}", event);
            return false
        }

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
            self.batch.push(json);
            self.crt_size_bytes += json_size;
        } else {
            log_error!("Failed to jsonify this event: {:?}", event);
            return false;
        }

        if self.is_need_flush() {
            self.write_to_file(0);
        }
        true
    }

    fn flush(self: &mut Self) {
        self.write_to_file(0)
    }

    fn close(self: &mut Self) {
        if !self.batch.is_empty() {
            self.write_to_file(0);
        }
    }
}

impl Default for LogConsumer {
    fn default() -> Self {
        LogConsumer::new("./log".to_string(), 100, None, None)
    }
}

impl Drop for LogConsumer {
    fn drop(&mut self) {
        self.close();
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