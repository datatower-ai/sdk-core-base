use serde_json::{json, Map, Value};
use crate::consumer::Consumer;
use crate::util::data_verification::verify_event;

/**
 * Should be run in a single thread for current implementation.
 */
struct LogConsumer {
    // Sets from outer
    path: String,
    max_batch_size: u32,
    name_prefix: String,
    max_file_size_bytes: u32,       // Max: 4 GB
    // Internally reserved
    file_size_bytes: u32,
    count: u32,
    batch: Vec<String>,
    file_time: i64,
}

impl LogConsumer {
    fn new(path: String, max_batch_size: u32, name_prefix: String, max_file_size_bytes: u32) -> Self {
        LogConsumer {
            path, max_batch_size, name_prefix, max_file_size_bytes,
            file_size_bytes: 0,
            count: 0,
            batch: Vec::new(),
            file_time: 0,
        }
    }

    fn is_need_flush(self: &Self) -> bool {
        self.file_size_bytes >= self.max_file_size_bytes
            || self.batch.len() as u32 >= self.max_batch_size
    }
}

impl Consumer for LogConsumer {
    fn add(mut self: &mut Self, event: Map<String, Value>) -> bool {
        if !verify_event(&event) {
            println!("[DT Core] Verification failed for this event: {:?}", event);
            return false
        }

        if let Ok(json) = serde_json::to_string(&event) {
            let json_size = json.len() as u32;
            if json_size + self.file_size_bytes > self.max_file_size_bytes {
                self.flush();
            }
            self.batch.push(json);
        } else {
            println!("[DT Core] Failed to jsonify this event: {:?}", event);
            return false;
        }

        if self.is_need_flush() {
            self.flush();
        }

        true
    }

    fn flush(mut self: &mut Self) {
        todo!()
    }

    fn close(mut self: Self) {
        todo!()
    }
}