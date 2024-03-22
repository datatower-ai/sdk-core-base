use std::backtrace::Backtrace;
use std::sync::Once;
use serde_json::{Map, Value};
use crate::base::mem;
use crate::base::MemValue::Consumer as MemConsumer;
use crate::consumer::Consumer;

pub mod util;
mod base;
pub mod consumer;
pub(crate) mod event;
pub(crate) mod upload;

static PANIC_HOOKER: Once = Once::new();

pub fn init_consumer(consumer: impl Consumer + 'static) -> bool {
    PANIC_HOOKER.call_once(|| {
        set_panic_hook();
    });

    let Ok(mut mem) = mem().lock() else {
        log_error!("Something wrong, lock is reentered!");
        return false;
    };

    if mem.contains_key(&consumer::MEM_KEY.to_string()) {
        log_error!("Consumer can only be initialized once.");
        false
    } else {
        mem.insert(consumer::MEM_KEY.to_string(), MemConsumer(Box::new(consumer)));
        true
    }
}

fn set_panic_hook() {
    use std::{panic::set_hook, process::exit};

    set_hook(Box::new(move |panic_info| {
        let backtrace = Backtrace::force_capture();
        let message = panic_info.to_string();
        eprintln!("Error: {}", message);
        eprintln!("Backtrace: {}", backtrace);
        exit(1);
    }));
}

pub fn add(event: Map<String, Value>) -> bool {
    let Ok(mut mem) = mem().lock() else {
        log_error!("Something wrong, lock is reentered!");
        return false;
    };
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        consumer.add(event)
    } else {
        log_error!("Consumer should be initialized before API calls!");
        false
    }
}

pub fn flush() {
    let Ok(mut mem) = mem().lock() else {
        log_error!("Something wrong, lock is reentered!");
        return;
    };
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        consumer.flush();
    } else {
        log_error!("Consumer should be initialized before API calls!");
    }
}

pub fn close() {
    let Ok(mut mem) = mem().lock() else {
        log_error!("Something wrong, lock is reentered!");
        return;
    };
    if let Some(MemConsumer(mut consumer)) = mem.remove(&consumer::MEM_KEY.to_string()) {
        consumer.close();
    } else {
        log_error!("Consumer should be initialized before API calls!");
    }
}