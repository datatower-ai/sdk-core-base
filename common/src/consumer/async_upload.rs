use std::cmp::{max, min};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use crate::consumer::Consumer;
use crate::{log_error};
use crate::event::BoxedEvent;
use crate::util::worker::worker::WorkerManager;
use crate::util::error::Result;

struct AsyncUploadConsumer {
    cache: Arc<Mutex<VecDeque<BoxedEvent>>>,
    worker_manager: WorkerManager,
    flushing_process_count: Arc<Mutex<USizeHolder>>,
    max_batch_size: usize
}

struct USizeHolder(usize);

impl AsyncUploadConsumer {
    fn new(num_threads: usize, max_batch_size: usize) -> Self {
        AsyncUploadConsumer {
            cache: Arc::new(Mutex::new(VecDeque::new())),
            worker_manager: WorkerManager::new(
                String::from("AsyncUploadConsumer#uploader"),
                min(1, num_threads)
            ),
            flushing_process_count: Arc::new(Mutex::new(USizeHolder(0))),
            max_batch_size,
        }
    }

    fn add_to_cache(self: &mut Self, event: BoxedEvent) -> Result<()> {
        {
            self.cache.lock().unwrap().push_back(event);
        }

        let fc = self.flushing_process_count.clone();
        if let Ok(mut count) = fc.lock() {
            if count.0 < self.worker_manager.size() {
                count.0 += 1;
                let _ = self.flush();
            } else {
                // Eliminates unnecessary duplicated flush() calls.
            }
        } else {
            let _ = self.flush();
        }

        Ok(())
    }

    fn upload_cache(self: &mut Self) {
        let cache = self.cache.clone();
        let count = self.flushing_process_count.clone();
        let max_batch_size = self.max_batch_size;

        self.worker_manager.schedule(move || {
            if let Ok(mut count) = count.lock() {
                count.0 = max(0, count.0 - 1);
            }

            let cache: Vec<BoxedEvent> = if let Ok(mut cache) = cache.lock() {
                if cache.is_empty() {
                    return;
                }

                let size = min(cache.len(), max_batch_size);
                let mut tmp = Vec::with_capacity(size);
                for _ in 0..size {
                    if let Some(event) = cache.pop_front() {
                        tmp.push(event)
                    } else {
                        break
                    }
                }
                tmp
            } else {
                // nothing to sent
                return;
            };

            let data_json = cache.iter()
                .filter_map(|it| {
                    if let Ok(json) = serde_json::to_string(it) {
                        Some(json)
                    } else {
                        log_error!("Failed to jsonify the given event: {:?}", it);
                        None
                    }
                }).collect::<Vec<String>>()
                .join(",");
            let data = format!("[{}]", data_json);
            println!("data: ({}) {}", cache.len(), data);
            // upload!
            sleep(Duration::from_millis(100));
        });
    }
}

impl Consumer for AsyncUploadConsumer {
    fn add(self: &mut Self, event: BoxedEvent) -> Result<()> {
        self.add_to_cache(event)
    }

    fn flush(self: &mut Self) -> Result<()> {
        self.upload_cache();
        Ok(())
    }

    fn close(self: &mut Self) -> Result<()> {
        self.worker_manager.shutdown();
        Ok(())
    }
}

impl Drop for AsyncUploadConsumer {
    fn drop(&mut self) {
        let _ = self.close();
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
        let mut c = AsyncUploadConsumer::new(2, 20);
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
                    let _ = c.add(Box::new(m));
                }
                _ => {}
            }
        }
    }
}