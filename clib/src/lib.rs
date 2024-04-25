use std::ffi::{c_char, CStr};
use std::sync::atomic::Ordering;
use serde_json::{Map, Value};
use common::log_error;
use common::util::error::DTError;
use common::util::error::DTError::HostError;
use common::util::result::{dissolve, dissolve_bool};
use common::util::error::Result;

#[no_mangle]
pub extern "C" fn dt_init(raw_config: *const c_char) -> i8 {
    let mut map = match cchar2map(raw_config) {
        Ok(map) => map,
        Err(e) => {
            log_error!("{e}");
            return 0;
        }
    };

    if let Some(Value::Number(number)) = map.get("_debug") {
        if let Some(number) = number.as_u64() {
            map.insert(String::from("_debug"), Value::from(number != 0));
        }
    }

    let success = dissolve_bool::<(), DTError>(common::init_by_config(map)).unwrap();
    if success {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn dt_add_event(raw_event: *const c_char) -> i8 {
    let map = match cchar2map(raw_event) {
        Ok(map) => map,
        Err(e) => {
            log_error!("{e}");
            return 0;
        }
    };

    let success = dissolve_bool::<(), DTError>(common::add(map)).unwrap();
    if success {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn dt_flush() {
    dissolve::<(), DTError>(common::flush()).unwrap();
}

#[no_mangle]
pub extern "C" fn dt_close() {
    dissolve::<(), DTError>(common::close()).unwrap();
}

#[no_mangle]
pub extern "C" fn dt_toggle_logger(enable: u8) {
    common::util::logger::LOG_ENABLED.store(enable != 0, Ordering::Relaxed);
}

fn cchar2map(cc: *const c_char) -> Result<Map<String, Value>> {
    let cstr = unsafe { CStr::from_ptr(cc) };
    let ss = match cstr.to_str() {
        Ok(config) => config,
        Err(e) => {
            return Err(HostError(e.to_string()));
        }
    };
    let json_result = serde_json::from_str(ss);
    match json_result {
        Ok(result) => match result {
            Value::Object(map) => Ok(map),
            _ => return Err(HostError(format!("Failed to parse init config! Given: {:?}", result))),
        },
        Err(e) => {
            return Err(HostError(format!("{}", e.to_string())));
        }
    }
}
