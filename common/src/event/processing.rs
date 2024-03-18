use std::sync::OnceLock;
use serde_json::{Map, Value};
use crate::event::data_verification::verify_event;

pub fn process_event(event_map: &mut Map<String, Value>) -> bool {
    inject_sdk_base_version(event_map);
    verify_event(event_map)
}

fn get_base_version() -> &'static str {
    static SDK_BASE_VERSION: OnceLock<&'static str> = OnceLock::new();
    SDK_BASE_VERSION.get_or_init(|| {
        env!("CARGO_PKG_VERSION")
    })
}

fn inject_sdk_base_version(event_map: &mut Map<String, Value>) {
    if let Some(Value::Object(properties)) = event_map.get_mut(&String::from("properties")) {
        let sdk_version = if let Some(Value::String(version)) = properties.remove(&String::from("#sdk_version_name")) {
            version
        } else {
            String::new()
        };
        let new_version = format!("{}_{}", sdk_version, get_base_version());
        properties.insert(String::from("#sdk_version_name"), Value::String(new_version));
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use super::{inject_sdk_base_version};

    #[test]
    fn its_work() {
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
        inject_sdk_base_version(j);
        println!("After injected: {:?}", j);
    }
}