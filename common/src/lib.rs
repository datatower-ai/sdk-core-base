use serde_json::{Map, Value};
use crate::base::mem;
use crate::base::MemValue::Consumer as MemConsumer;
use crate::consumer::Consumer;

pub mod util;
mod base;
pub mod consumer;
pub mod event;

pub fn init_consumer(consumer: impl Consumer + 'static) -> bool {
    let mut mem = mem().lock().unwrap();
    if mem.contains_key(&consumer::MEM_KEY.to_string()) {
        log_error!("Consumer can only be initialized once.");
        false
    } else {
        mem.insert(consumer::MEM_KEY.to_string(), MemConsumer(Box::new(consumer)));
        true
    }
}

pub fn add(event: Map<String, Value>) -> bool {
    let mut mem = mem().lock().unwrap();
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        consumer.add(event)
    } else {
        log_error!("Consumer should be initialized before API calls!");
        false
    }
}

pub fn flush() {
    let mut mem = mem().lock().unwrap();
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        consumer.flush();
    } else {
        log_error!("Consumer should be initialized before API calls!");
    }
}

pub fn close() {
    let mut mem = mem().lock().unwrap();
    if let Some(MemConsumer(mut consumer)) = mem.remove(&consumer::MEM_KEY.to_string()) {
        consumer.close();
    } else {
        log_error!("Consumer should be initialized before API calls!");
    }
}