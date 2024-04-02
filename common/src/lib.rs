use std::backtrace::Backtrace;
use std::sync::atomic::Ordering;
use std::sync::Once;
use serde_json::{Map, Value};
use crate::base::mem;
use crate::base::MemValue::Consumer as MemConsumer;
use crate::consumer::Consumer;
use crate::consumer::log::LogConsumer;
use crate::event::common_properties::{clear_static_comm_props, Props, set_static_comm_props};
use crate::event::Event;
use crate::event::processing::{DEBUG, process_event};
use crate::util::error::{DTError, Result};
use crate::util::error::macros::{host_error, internal_error, runtime_error};

pub mod util;
mod base;
pub mod consumer;
pub mod event;
pub(crate) mod upload;

static INITIALIZER: Once = Once::new();

pub fn init_by_config(mut config: Map<String, Value>) -> Result<()> {
    let Some(Value::String(cn)) = config.get("consumer") else {
        return host_error!("Initialization config is missing 'consumer' or its type is not valid!")
    };

    let consumer: Result<Box<dyn Consumer>> = match cn.to_lowercase().as_str() {
        "log" => LogConsumer::from_config(&mut config),
        _ => return host_error!("Initialization config has 'consumer' but it's out of domain!")
    };

    init_consumer(consumer?)?;

    // after init success:
    if let Some(Value::Bool(debug)) = config.get("_debug") {
        DEBUG.store(*debug, Ordering::Relaxed);
    }

    Ok(())
}

pub fn init_consumer(consumer: Box<dyn Consumer>) -> Result<()> {
    INITIALIZER.call_once(|| {
        set_panic_hook();
        if let Err(e) = event::init() {
            log_error!("Failed to init event processor, reason: {e}")
        }
    });

    let Ok(mut mem) = mem().lock() else {
        return internal_error!("lock is reentered!");
    };

    if mem.contains_key(&consumer::MEM_KEY.to_string()) {
        runtime_error!("Consumer can only be initialized once.")
    } else {
        mem.insert(consumer::MEM_KEY.to_string(), MemConsumer(consumer));
        log_info!("Initialized!");
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
        let ret = consumer.close();
        log_info!("Closed!");
        ret
    } else {
        runtime_error!("Consumer should be initialized before API calls!")
    }
}

pub fn set_static_common_props(props: Props) -> Result<()> {
    set_static_comm_props(props)
}

pub fn clear_static_common_props() -> Result<()> {
    clear_static_comm_props()
}