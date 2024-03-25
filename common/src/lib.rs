use std::backtrace::Backtrace;
use std::sync::Once;
use crate::base::mem;
use crate::base::MemValue::Consumer as MemConsumer;
use crate::consumer::Consumer;
use crate::event::Event;
use crate::event::processing::process_event;
use crate::util::error::{DTError, Result};
use crate::util::error::macros::{internal_error, runtime_error};

pub mod util;
mod base;
pub mod consumer;
pub mod event;
pub(crate) mod upload;

static PANIC_HOOKER: Once = Once::new();

pub fn init_consumer(consumer: impl Consumer + 'static) -> Result<()> {
    PANIC_HOOKER.call_once(|| {
        set_panic_hook();
    });

    let Ok(mut mem) = mem().lock() else {
        return internal_error!("lock is reentered!");
    };

    if mem.contains_key(&consumer::MEM_KEY.to_string()) {
        runtime_error!("Consumer can only be initialized once.")
    } else {
        mem.insert(consumer::MEM_KEY.to_string(), MemConsumer(Box::new(consumer)));
        Ok(())
    }
}

fn set_panic_hook() {
    use std::{panic::set_hook, process::exit};

    set_hook(Box::new(move |panic_info| {
        // Notice: Panics are for unrecoverable and unexpected errors!
        let backtrace = Backtrace::force_capture();
        let message = panic_info.to_string();
        eprintln!("Error: {}", message);
        eprintln!("Backtrace: {}", backtrace);
        exit(1);
    }));
}

pub fn add(event: Event) -> Result<()> {
    let Ok(mut mem) = mem().lock() else {
        return internal_error!("lock is reentered!");
    };
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        let event = process_event(event)?;
        consumer.add(Box::new(event))
    } else {
        runtime_error!("Consumer should be initialized before API calls!")
    }
}

pub fn flush() -> Result<()> {
    let Ok(mut mem) = mem().lock() else {
        return internal_error!("Something wrong, lock is reentered!");
    };
    if let Some(MemConsumer(consumer)) = mem.get_mut(&consumer::MEM_KEY.to_string()) {
        consumer.flush()
    } else {
        runtime_error!("Consumer should be initialized before API calls!")
    }
}

pub fn close() -> Result<()> {
    let Ok(mut mem) = mem().lock() else {
        return internal_error!("Something wrong, lock is reentered!");
    };
    if let Some(MemConsumer(mut consumer)) = mem.remove(&consumer::MEM_KEY.to_string()) {
        consumer.close()
    } else {
        runtime_error!("Consumer should be initialized before API calls!")
    }
}