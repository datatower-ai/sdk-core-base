use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::log_warning;

#[cfg(feature = "benchmark")]
pub static BM_TRACER: Lazy<BenchmarkTracer> = Lazy::new(|| BenchmarkTracer::new());

#[cfg(feature = "benchmark")]
pub struct BenchmarkTracer {
    holder: Mutex<HashMap<String, Vec<u128>>>
}

impl BenchmarkTracer {
    pub fn new() -> Self {
        BenchmarkTracer {
            holder: Mutex::new(HashMap::new())
        }
    }

    pub fn add(self: &Self, key: &str, value: u128) {
        let mut guard = self.holder.lock().unwrap();
        if let Some(vec) = guard.get_mut(key) {
            vec.push(value);
        } else {
            guard.insert(key.to_string(), vec![value]);
        }
    }

    pub fn summary(self: &Self) {
        let guard = self.holder.lock().unwrap();
        for (key, vec) in guard.iter() {
            let sum: u128 = vec.iter().sum();
            let avg = sum / vec.len() as u128;
            let max = vec.iter().max().unwrap_or(&0);
            let min = vec.iter().min().unwrap_or(&0);
            log_warning!("{}: sum: {}µs ({}), avg: {}µs, max: {:?}µs, min: {:?}µs", key, sum, vec.len(), avg, max, min)
        }
    }
}
