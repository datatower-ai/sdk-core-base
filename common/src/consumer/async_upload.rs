use std::cmp::min;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use serde_json::{Map, Value};
use crate::consumer::Consumer;
use crate::event::processing::process_event;
use crate::{log_error};
use crate::util::worker::worker::WorkerManager;

struct AsyncUploadConsumer {
    cache: Arc<Mutex<VecDeque<String>>>,
    upload_wm: WorkerManager,
}

impl AsyncUploadConsumer {
    fn new(num_upload_threads: usize) -> Self {
        AsyncUploadConsumer {
            cache: Arc::new(Mutex::new(VecDeque::new())),
            upload_wm: WorkerManager::new(
                String::from("AsyncUploadConsumer#uploader"),
                min(1, num_upload_threads)
            ),
        }
    }
}

impl Consumer for AsyncUploadConsumer {
    fn add(self: &mut Self, mut event: Map<String, Value>) -> bool {
        if !process_event(&mut event) {
            log_error!("Verification failed for this event: {:?}", event);
            return false;
        }

        if let Ok(json_str) = serde_json::to_string(&event) {
            self.cache.lock().unwrap().push_back(json_str);

            self.flush();
            true
        } else {
            log_error!("Failed to jsonify the given event: {:?}", event);
            false
        }
    }

    fn flush(self: &mut Self) {
        let cache = self.cache.clone();

        self.format_wm.schedule(move || {
            let cache: Vec<String> = if let Ok(mut cache) = cache.lock() {
                let mut tmp = Vec::with_capacity(cache.len());
                loop {
                    if let Some(event) = cache.pop_front() {
                        tmp.push(event)
                    } else {
                        break
                    }
                }
                tmp
            } else {
                vec![]
            };

            if cache.is_empty() {
                return;
            };

            let data = format!("[{}]", cache.join(","));
            println!("data: ({}) {}", cache.len(), data);
            // upload!
            sleep(Duration::from_millis(30));
        });
    }

    fn close(self: &mut Self) {
        self.format_wm.shutdown();
    }
}

impl Drop for AsyncUploadConsumer {
    fn drop(&mut self) {
        self.close();
    }
}

unsafe impl Send for AsyncUploadConsumer {}
unsafe impl Sync for AsyncUploadConsumer {}

#[cfg(test)]
mod test {
    use serde_json::{json, Value};
    use crate::consumer::async_upload::AsyncUploadConsumer;
    use crate::consumer::Consumer;

    #[test]
    fn it_works() {
        let mut c = AsyncUploadConsumer::new(2);

        for i in 0..=50 {
            let j = json!({
            "#app_id": "123",
            "#event_time": i,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#sdk_version_name": "1.2.3",
                "a": [1, 2, 3]
            }
        });
            match j {
                Value::Object(m) => {
                    c.add(m);
                }
                _ => {}
            }
        }
    }
}