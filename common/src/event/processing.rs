use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{Map, Number, Value};
use crate::event::data_verification::{META_PROPS, verify_event};
use crate::event::Event;
use crate::log_error;
use crate::util::error::{DTError, Result};
use crate::util::error::DTError::InternalError;
use crate::util::error::macros::error_with;

pub fn process_event(event_map: Event) -> Result<Event> {
    let mut event = roughen_event(event_map)?;
    fulfill_metas(&mut event);
    inject_sdk_base_info(&mut event);
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

fn roughen_event(mut event: Event) -> Result<Event> {
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
}

fn inject_sdk_base_info(event_map: &mut Event) {
    let version_key: String = String::from("#sdk_version_name");
    let type_key: String = String::from("#sdk_type");

    if let Some(Value::Object(properties)) = event_map.get_mut(&String::from("properties")) {
        if let Some(Value::String(version)) = properties.get(&version_key) {
            let new_version = format!("{}_{}", version, get_base_version());
            properties.insert(version_key, Value::String(new_version));
        } else {
            log_error!("{}", InternalError(String::from("⚠ CAUTION! Forget to set #sdk_version_name?")));
            let new_version = format!("unknown_{}", get_base_version());
            properties.insert(version_key, Value::String(new_version));
        };

        if let Some(Value::String(_)) = properties.get(&type_key) {} else {
            log_error!("{}", InternalError(String::from("⚠ CAUTION! Forget to set #sdk_type?")));
            properties.insert(type_key, Value::String(String::from("dt_core_base")));
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use super::{inject_sdk_base_info, roughen_event};

    #[test]
    fn inject_sdk_base_info() {
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
    fn roughen_event_loop_test() {
        let st = std::time::Instant::now();
        for _ in 0..1000 {
            roughen_event_test()
        }
        println!("Total: {}", st.elapsed().as_micros())
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
        match roughen_event(j) {
            Ok(x) => println!("{:?}", x),
            Err(e) => eprintln!("{e}"),
        }
        println!("{}", st.elapsed().as_micros());
    }
}