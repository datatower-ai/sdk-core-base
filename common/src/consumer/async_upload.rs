use std::sync::Mutex;
use serde_json::{Map, Value};
use crate::consumer::Consumer;
use crate::event::processing::process_event;
use crate::log_error;
use crate::util::worker::worker::WorkerManager;

struct AsyncUploadConsumer {
    cache: Mutex<Vec<String>>,
    worker_manager: WorkerManager,
}

impl AsyncUploadConsumer {
    fn new(num_threads: usize) -> Self {
        AsyncUploadConsumer {
            cache: Mutex::new(Vec::new()),
            worker_manager: WorkerManager::new(
                String::from("AsyncUploadConsumer"),
                num_threads
            )
        }
    }
}

impl Consumer for AsyncUploadConsumer {
    fn add(self: &mut Self, mut event: Map<String, Value>) -> bool {
        if !process_event(&mut event) {
            log_error!("Verification failed for this event: {:?}", event);
            return false
        }
        self.flush();
        true
    }

    fn flush(self: &mut Self) {
        // self.worker_manager.schedule(|| {
        //     let cache = if let Ok(cache) = self.cache.lock() {
        //         cache
        //     } else {
        //         log_error!("Failed to get cache while flush!");
        //         return;
        //     };
        //     if cache.is_empty() {
        //         return;
        //     };
        //
        //     let mut data = "";
        // });
    }

    fn close(self: &mut Self) {
        self.worker_manager.shutdown();
    }
}

impl Drop for AsyncUploadConsumer {
    fn drop(&mut self) {
        self.close();
    }
}