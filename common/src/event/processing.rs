use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{Map, Number, Value};
use crate::event::common_properties::fulfill_by_comm_props;
use crate::event::data_verification::{META_PROPS, verify_event};
use crate::event::Event;
use crate::log_error;
use crate::util::error::{DTError, Result};
use crate::util::error::DTError::InternalError;
use crate::util::error::macros::error_with;

pub static DEBUG: AtomicBool = AtomicBool::new(false);

pub fn process_event(event_map: Event) -> Result<Event> {
    let mut event = eventify(event_map)?;
    fulfill_metas(&mut event);
    inject_sdk_base_info(&mut event);
    fulfill_by_comm_props(&mut event)?;
    let verify_result = verify_event(&mut event);

    match verify_result {
        Err(e) => if let DTError::VerifyError(_) = e {
            error_with!(e, "Verification failed for {event:?}")
        } else {
            Err(e)
        },
        Ok(_) => Ok(event)
    }
}

fn is_need_eventify(event: &Event) -> bool {
    if event.len() > META_PROPS.len() {
        // Guarantees to contain non-meta properties.
        true
    } else {
        for key in event.keys() {
            if !META_PROPS.contains_key(key.as_str()) {
                // contains non-meta properties
                return true;
            }
        }
        // Only metas are presented
        false
    }
}

/// flatted map / event -> event
fn eventify(mut event: Event) -> Result<Event> {
    if !is_need_eventify(&event) {
        return Ok(event);
    }

    let mut result: Map<String, Value> = Map::with_capacity(META_PROPS.len());
    // Takes meta out.
    for k in META_PROPS.keys() {
        let k = *k;
        if let Some(v) = event.remove(k) {
            result.insert(String::from(k), v);
        }
    }
    // Makes event as a nested properties.
    result.insert(String::from("properties"), Value::Object(event));
    Ok(result)
}

fn get_base_version() -> &'static str {
    static SDK_BASE_VERSION: OnceLock<&'static str> = OnceLock::new();
    SDK_BASE_VERSION.get_or_init(|| {
        env!("CARGO_PKG_VERSION")
    })
}

fn fulfill_metas(event: &mut Event) {
    if !event.contains_key("#event_time") {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_millis() as u64;
        event.insert(String::from("#event_time"), Value::Number(Number::from(time)));
    }

    if !event.contains_key("#event_syn") {
        event.insert(String::from("#event_syn"), Value::String(uuid::Uuid::new_v4().to_string()));
    }

    if DEBUG.load(Ordering::Relaxed) {
        event.insert(String::from("#debug"), Value::from(true));
    }

    if !event.contains_key(&String::from("properties")) {
        event.insert(String::from("properties"), Value::Object(serde_json::Map::with_capacity(2)));
    }
}

fn inject_sdk_base_info(event_map: &mut Event) {
    let type_key: String = String::from("#sdk_type");

    if let Some(Value::Object(properties)) = event_map.get_mut(&String::from("properties")) {
        let version = get_base_version().to_string();
        properties.insert(String::from("#sdk_version_name"), Value::String(version));

        if let Some(Value::String(_)) = properties.get(&type_key) {} else {
            log_error!("{}", InternalError(String::from("⚠ CAUTION! Forget to set #sdk_type?")));
            properties.insert(type_key, Value::String(String::from("dt_core_base")));
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use super::{inject_sdk_base_info, eventify, process_event};

    #[test]
    fn test_inject_sdk_base_info() {
        let mut j = json!({
            "#app_id": "123",
            "#event_time": 123,
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
        let j = j.as_object_mut().unwrap();
        inject_sdk_base_info(j);
        println!("After injected: {:?}", j);
    }

    #[test]
    fn roughen_event_test() {
        let mut j = json!({
            "#app_id": "123",
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#sdk_version_name": "1.2.3",
            "productNames": ["Lua", "hello"],
            "productType": "Lua book",
            "producePrice": 80,
            "shop": "xx-shop",
            "#os": "1.1.1.1",
            "date": 111,
            "date1": 111,
            "sex": "female"
        });
        let j = j.as_object_mut().unwrap().to_owned();
        let st = std::time::Instant::now();
        match eventify(j) {
            Ok(_x) => {},//println!("{:?}", x),
            Err(e) => eprintln!("{e}"),
        }
        println!("{}", st.elapsed().as_micros());
    }

    #[test]
    fn benchmark() {
        crate::event::data_verification::init().expect("Failed to init");
        let n = 100000;

        let mut j = json!({
                "#app_id": "appid_1234567890",
                "#dt_id": "1234567890987654321",
                "#bundle_id": "com.example",
                "#event_name": "test_event",
                "#event_type": "track",
                "#sdk_type": "rust",
                "#sdk_version_name": "0.0.0",
                "productNames": ["Lua", "hello"],
                "productType": "Lua book",
                "producePrice": 80,
                "shop": "xx-shop",
                "#os": "1.1.1.1",
                "sex": "female"
            });
        let j = j.as_object_mut().unwrap().to_owned();

        let mut tm = 0;
        for _ in 0..n {
            let st = std::time::Instant::now();
            process_event(j.clone()).expect("This event is not valid");
            tm += st.elapsed().as_micros();
        }
        println!("Total: {}, Avg: {}", tm, tm / n);
        println!("QPS: {}", 1000000/(tm/n));
    }
}