use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Map, Value};

use crate::event::Event;
use crate::util::error::macros::verify_error;
use crate::util::error::Result;

const NAME_REGEX_STR: &'static str = r"^[a-zA-Z#][a-zA-Z\d_]{0,63}$";
static NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(NAME_REGEX_STR).unwrap());

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum TypeConstraint {
    String,
    Number,             // Integer + Float
    Integer,
    Float,
    Bool,
    Object,
    Array,
}

type PropsConstraintMap = Lazy<HashMap<&'static str, TypeConstraint>>;

pub(super) static META_PROPS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#app_id", TypeConstraint::String), ("#bundle_id", TypeConstraint::String),
    ("#android_id", TypeConstraint::String), ("#gaid", TypeConstraint::String),
    ("#dt_id", TypeConstraint::String), ("#acid", TypeConstraint::String),
    ("#event_name", TypeConstraint::String), ("#event_type", TypeConstraint::String),
    ("#event_time", TypeConstraint::Integer), ("#event_syn", TypeConstraint::String),
    ("properties", TypeConstraint::Object), ("#debug", TypeConstraint::Bool),
]));
pub(super) static COMPULSORY_META_PROPS: Lazy<Vec<String>> = Lazy::new(|| vec!(
    String::from("#app_id"), String::from("#bundle_id"),
    String::from("#dt_id"), String::from("#event_time"),
    String::from("#event_name"), String::from("#event_type"),
    String::from("#event_syn"), String::from("properties")
));

static COMMON_PROPS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#sdk_type", TypeConstraint::String), ("#sdk_version_name", TypeConstraint::String)
]));
static PRESET_EVENT_PROPS_COMMON: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#bundle_id", TypeConstraint::String), ("#zone_offset", TypeConstraint::Number),
    ("#session_id", TypeConstraint::String), ("#device_manufacturer", TypeConstraint::String),
    ("#is_foreground", TypeConstraint::Bool), ("#mcc", TypeConstraint::String),
    ("#mnc", TypeConstraint::String), ("#os_country_code", TypeConstraint::String),
    ("#os_lang_code", TypeConstraint::String), ("#app_version_code", TypeConstraint::Integer),
    ("#app_version_name", TypeConstraint::String), ("#os", TypeConstraint::String),
    ("#os_version_name", TypeConstraint::String), ("#os_version_code", TypeConstraint::Number),
    ("#device_brand", TypeConstraint::String), ("#device_model", TypeConstraint::String),
    ("#screen_height", TypeConstraint::Number), ("#screen_width", TypeConstraint::Number),
    ("#memory_used", TypeConstraint::String), ("#storage_used", TypeConstraint::String),
    ("#network_type", TypeConstraint::String), ("#simulator", TypeConstraint::Bool),
    ("#fps", TypeConstraint::Number), ("#scene", TypeConstraint::String),
    ("#mp_platform", TypeConstraint::String), ("#gaid", TypeConstraint::String),
    ("#build_device", TypeConstraint::String), ("#duration", TypeConstraint::String),
    ("#firebase_iid", TypeConstraint::String), ("#appsflyer_id", TypeConstraint::String),
    ("#adjust_id", TypeConstraint::String), ("#kochava_id", TypeConstraint::String),
]));
static PRESET_PROPS_USER_COMMON: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#active_device_model", TypeConstraint::String), ("#active_network_type", TypeConstraint::String),
    ("#active_os_version_name", TypeConstraint::String), ("#active_os", TypeConstraint::String),
    ("#active_os_lang_code", TypeConstraint::String), ("#firebase_iid", TypeConstraint::String),
    ("#active_bundle_id", TypeConstraint::String), ("#active_device_manufacturer", TypeConstraint::String),
    ("#active_screen_width", TypeConstraint::Number), ("#active_mcc", TypeConstraint::String),
    ("#active_os_country_code", TypeConstraint::String), ("#active_mnc", TypeConstraint::String),
    ("#active_storage_used", TypeConstraint::String), ("#active_user_agent", TypeConstraint::String),
    ("#active_app_version_code", TypeConstraint::Number), ("#active_sdk_type", TypeConstraint::String),
    ("#active_device_brand", TypeConstraint::String), ("#active_memory_used", TypeConstraint::String),
    ("#active_sdk_version_name", TypeConstraint::String), ("#active_screen_height", TypeConstraint::Number),
    ("#active_app_version_name", TypeConstraint::String), ("#active_simulator", TypeConstraint::Bool),
]));
static PRESET_PROPS_AD: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_seq", TypeConstraint::String), ("#ad_id", TypeConstraint::String),
    ("#ad_type_code", TypeConstraint::Integer), ("#ad_platform_code", TypeConstraint::Integer),
    ("#ad_mediation_code", TypeConstraint::Integer), ("#ad_mediation_id", TypeConstraint::String),
]));
static PRESET_PROPS_IAS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ias_original_order", TypeConstraint::String), ("#ias_order", TypeConstraint::String),
    ("#ias_sku", TypeConstraint::String), ("#ias_price", TypeConstraint::Number),
    ("#ias_currency", TypeConstraint::String)
]));
static PRESET_PROPS_APP_INSTALL: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#referrer_url", TypeConstraint::String), ("#referrer_click_time", TypeConstraint::Integer),
    ("#referrer_click_time_server", TypeConstraint::Integer), ("#app_install_time", TypeConstraint::Integer),
    ("#app_install_time_server", TypeConstraint::Integer), ("#instant_experience_launched", TypeConstraint::Bool),
    ("#failed_reason", TypeConstraint::String), ("#cnl", TypeConstraint::String)
]));
static PRESET_PROPS_SESSION_START: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#is_first_time", TypeConstraint::Bool), ("#resume_from_background", TypeConstraint::Bool),
    ("#start_reason", TypeConstraint::String), ("#background_duration", TypeConstraint::Integer)
]));
static PRESET_PROPS_SESSION_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#session_duration", TypeConstraint::Integer)
]));
static PRESET_PROPS_IAP_PURCHASE_SUCCESS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#iap_order", TypeConstraint::String), ("#iap_sku", TypeConstraint::String),
    ("#iap_price", TypeConstraint::Number), ("#iap_currency", TypeConstraint::String)
]));
static PRESET_PROPS_AD_EXCEPT_BEGIN_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_entrance", TypeConstraint::String), ("#ad_location", TypeConstraint::String)
]));
static PRESET_PROPS_AD_LOAD_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#load_result", TypeConstraint::Bool), ("#load_duration", TypeConstraint::Number),
]));
static PRESET_PROPS_AD_FAILED_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#error_code", TypeConstraint::Integer), ("#error_message", TypeConstraint::String),
]));
static PRESET_PROPS_AD_PAID: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_value", TypeConstraint::Number), ("#ad_currency", TypeConstraint::String),
    ("#ad_precision", TypeConstraint::String), ("#ad_country_code", TypeConstraint::String),
]));
static PRESET_PROPS_AD_CONVERSION: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_conversion_source", TypeConstraint::String)
]));
static PRESET_EVENTS: Lazy<HashMap<&str, Vec<&PropsConstraintMap>>> = Lazy::new(|| HashMap::from([
    ("#app_install", vec![&PRESET_PROPS_APP_INSTALL]),
    ("#session_start", vec![&PRESET_PROPS_SESSION_START]),
    ("#session_end", vec![&PRESET_PROPS_SESSION_END]),
    ("#ad_load_begin", vec![&PRESET_PROPS_AD]),
    ("#ad_load_end", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_LOAD_END, &PRESET_PROPS_AD_FAILED_END]),
    ("#ad_to_show", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END]),
    ("#ad_show", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END]),
    ("#ad_show_failed", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END, &PRESET_PROPS_AD_FAILED_END]),
    ("#ad_close", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END]),
    ("#ad_click", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END]),
    ("#ad_rewarded", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END]),
    ("#ad_conversion", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END, &PRESET_PROPS_AD_CONVERSION]),
    ("#ad_paid", vec![&PRESET_PROPS_AD, &PRESET_PROPS_AD_EXCEPT_BEGIN_END, &PRESET_PROPS_AD_PAID]),
    ("#iap_purchase_success", vec![&PRESET_PROPS_IAP_PURCHASE_SUCCESS]),
    ("#ias_subscribe_success", vec![&PRESET_PROPS_IAS]),
    ("#ias_subscribe_notify", vec![&PRESET_PROPS_IAS]),
]));

pub(super) fn init() -> Result<()> {
    // Init the regex beforehand.
    let _ = NAME_RE.is_match("a");
    Ok(())
}

pub(crate) fn verify_event(event_map: &Event) -> Result<()> {
    for prop in COMPULSORY_META_PROPS.iter() {
        if let Some(value) = event_map.get(prop) {
            if let Some(constraint) = META_PROPS.get(prop.as_str()) {
                if !check_type_constraint(value, constraint) {
                    return verify_error!(
                        "Type of value of meta property is not valid! Expected: {:?}, got: {}", constraint, value
                    );
                }
            }
        } else {
            return verify_error!("Meta property \"{}\" is required, but missing!", prop);
        }
    }

    check_meta_is_string_and_nonempty(event_map, String::from("#app_id"))?;
    check_meta_is_string_and_nonempty(event_map, String::from("#dt_id"))?;

    let Some(Value::String(event_name)) = event_map.get("#event_name") else {
        return verify_error!("#event_name is missing or it's type is invalid!");
    };

    if !NAME_RE.is_match(event_name) {
        return verify_error!("#event_name must be a valid variable name!");
    }

    let Some(Value::String(event_type)) = event_map.get("#event_type") else {
        return verify_error!("#event_type is missing or it's type is invalid!");
    };

    let Some(Value::Object(properties)) = event_map.get("properties") else {
        return verify_error!("properties is missing or it's type is invalid!");
    };

    if event_type == "track" {
        if is_preset(event_name) {
            if let Some(props_list) = PRESET_EVENTS.get(event_name.as_str()) {
                verify_preset_event(event_name, properties, props_list)
            } else {
                return verify_error!("event_name (\"{}\") is out of scope (preset)!", event_name);
            }
        } else {
            verify_custom_event(event_name, properties)
        }
    } else if event_type == "user" {
        verify_user_event(event_name, properties)
    } else {
        return verify_error!("event_type (\"{}\") is invalid!", event_type);
    }
}

fn is_preset(name: &String) -> bool {
    name.starts_with("#")
}

fn check_type_constraint(value: &Value, target: &TypeConstraint) -> bool {
    match value {
        Value::String(_) => *target == TypeConstraint::String,
        Value::Bool(_) => *target == TypeConstraint::Bool,
        Value::Object(_) => *target == TypeConstraint::Object,
        Value::Array(_) => *target == TypeConstraint::Array,
        Value::Number(n) => {
            if *target == TypeConstraint::Number {
                true
            } else if n.is_i64() || n.is_u64() {
                *target == TypeConstraint::Integer
            } else {
                *target == TypeConstraint::Float
            }
        }
        _ => false
    }
}

fn check_meta_is_string_and_nonempty(event_map: &Event, key: String) -> Result<()> {
    if let Some(value) = event_map.get(&key) {
        if let Value::String(value) = value {
            if value.len() == 0 {
                verify_error!("{} cannot be empty!", key)
            } else {
                Ok(())
            }
        } else {
            verify_error!("{} should be a string!", key)
        }
    } else {
        verify_error!("{} is required, but missing", key)
    }
}

fn verify_preset_event(
    event_name: &String,
    properties: &Map<String, Value>,
    props_list: &Vec<&PropsConstraintMap>
) -> Result<()> {
    for (key, value) in properties {
        verify_properties(event_name, key, value, find_constraint_for_event(key, props_list))?
    }
    Ok(())
}

fn verify_properties(
    event_name: &String,
    key: &String, value: &Value,
    type_constraint: Option<&TypeConstraint>
) -> Result<()> {
    if !NAME_RE.is_match(key) {
        return verify_error!("Property name (\"{}\") is invalid!", key);
    }

    if is_preset(key) {
        if let Some(constraint) = type_constraint {
            if !check_type_constraint(value, constraint) {
                verify_error!(
                    "The type of value for property \"{}\" is not valid (Given: {}, Expected: {:?})!",
                    key, value, constraint
                )
            } else {
                Ok(())
            }
        } else {
            // Property (starts with #) is out of scope.
            return verify_error!(
                "Key of property (\"{}\") is out of scope for event (\"{}\")!", key, event_name
            )
        }
    } else {
        // Custom properties (not starts with #) are allowed for all events.
        Ok(())
    }
}

fn find_constraint_for_user_event<'a>(
    prop_name: &str
) -> Option<&'a TypeConstraint> {
    PRESET_PROPS_USER_COMMON.get(prop_name).or(
        COMMON_PROPS.get(prop_name)
    )
}

fn find_constraint_for_event<'a>(
    prop_name: &str,
    constraints: &'a Vec<&PropsConstraintMap>
) -> Option<&'a TypeConstraint> {
    for map in constraints {
        if let Some(constraint) = map.get(prop_name) {
            return Some(constraint)
        }
    }
    PRESET_EVENT_PROPS_COMMON.get(prop_name).or(COMMON_PROPS.get(prop_name))
}

fn verify_user_event(event_name: &String, properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        verify_properties(event_name, k, v, find_constraint_for_user_event(k))?
    }

    if event_name == "#user_append" || event_name == "#user_uniq_append" {
        verify_all_custom_props_are_list(properties)
    } else if event_name == "#user_add" {
        verify_all_custom_props_are_num(properties)
    } else {
        Ok(())
    }
}

fn verify_custom_event(event_name: &String, properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        verify_properties(event_name, k, v, find_constraint_for_event(k, &Vec::with_capacity(0)))?
    }
    Ok(())
}

fn verify_all_custom_props_are_list(properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        if is_preset(k) {
            continue;
        }
        let Value::Array(_) = v else {
            return verify_error!("Type of value in this event should be List");
        };
    }
    Ok(())
}

fn verify_all_custom_props_are_num(properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        if is_preset(k) {
            continue;
        }
        let Value::Number(_) = v else {
            return verify_error!("Type of value in this event should be Number");
        };
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use serde_json::{json, Value};

    use super::verify_event;

    fn verify(obj: Value, target: bool) {
        let obj = obj.as_object().unwrap();
        let st = std::time::Instant::now();
        let pass = verify_event(obj);
        println!("{}µs, {}, {:?}", st.elapsed().as_micros(), pass.is_ok(), obj);
        assert_eq!(pass.is_ok(), target)
    }

    #[test]
    fn its_work() {
        super::init().expect("Failed to init");
        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, true);
    }

    #[test]
    fn missing_meta() {
        super::init().expect("Failed to init");
        let j = json!({
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": "eeeee"
        });
        verify(j, false);
    }

    #[test]
    fn wrong_type_meta() {
        super::init().expect("Failed to init");
        let j = json!({
            "#app_id": "123",
            "#event_time": "123",               // <- wrong type
            "#dt_id": true,                     // <- wrong type
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_syn": 123,              // <- wrong type
            "properties": {
                "a": [1, 2, 3]
            }
        });
        verify(j, false);
    }

    #[test]
    fn preset_event() {
        super::init().expect("Failed to init");
        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#session_start",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": true,
                "a": [1, 2, 3]
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#session_start",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": 1,       // <- wrong type
                "a": [1, 2, 3]
            }
        });
        verify(j, false);


        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#session_start",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": true,
                "a": [1, 2, 3],
                "$custom": "123"            // <- custom "preset" props with $
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#session_start",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": true,
                "a": [1, 2, 3],
                "#custom": "123"            // <- custom "preset" props with #
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#session_start",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": true,
                "a": [1, 2, 3],
                "#session_duration": 123                // <- out of scope
            }
        });
        verify(j, false);
    }

    #[test]
    fn custom_event() {
        super::init().expect("Failed to init");
        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "ccc_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#resume_from_background": true,            // <-
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "ccc_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#device_brand": "X",
                "a": [1, 2, 3]
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "ccc_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#device_brand": 1,                 // <- wrong type
                "a": [1, 2, 3]
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "ccc_event",
            "#event_type": "track",
            "#event_syn": "eeeee",
            "properties": {
                "#dccc": 1,                         // <- not preset props
                "a": {
                    "123": "xx"
                }
            }
        });
        verify(j, false);
    }

    #[test]
    fn user_event() {
        super::init().expect("Failed to init");
        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "1"
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "1",
                "#sdk_type": "xxx",
                "#sdk_version_name": "1.2.3"
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "1",
                "#firebase_iid": "123"
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "1",
                "#asd": 1                       // <- custom "preset" props
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_add",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": 1
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_add",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "123"                  // <- wrong type
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_append",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": ["abcdefg"]
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_append",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": 1                      // <- wrong type
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_uniq_append",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": ["abcdefg", "xxxxx"]
            }
        });
        verify(j, true);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_uniq_append",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": 1                      // <- wrong type
            }
        });
        verify(j, false);

        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": "1",
                "#is_foreground": false     // <- out of scope of user props
            }
        });
        verify(j, false);


        let j = json!({
            "#app_id": "123",
            "#event_time": 123,
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "#user_set",
            "#event_type": "user",
            "#event_syn": "eeeee",
            "properties": {
                "a": [1, 2, 3],
                "#os_country_code": false     // <- out of scope of user props
            }
        });
        verify(j, false);
    }

    #[test]
    fn benchmark() {
        super::init().expect("Failed to init");
        let n = 10000;

        let mut j = json!({
            "#app_id": "123",
            "#dt_id": "ddd",
            "#bundle_id": "com.xx",
            "#event_name": "test_event",
            "#event_type": "track",
            "#event_time": 0,
            "#event_syn": "x",
            "properties": {
                "#sdk_version_name": "1.2.3",
                "productNames": ["Lua", "hello"],
                "productType": "Lua book",
                "producePrice": 80,
                "shop": "xx-shop",
                "#os": "1.1.1.1",
                "date": 111,
                "date1": 111,
                "sex": "female"
            }
        });
        let j = j.as_object_mut().unwrap().to_owned();

        let st = std::time::Instant::now();
        for _ in 0..n {
            verify_event(&j).expect("This event is not valid");
        }
        let elapsed = st.elapsed().as_micros();
        println!("Total: {}, Avg: {}", elapsed, elapsed / n)
    }
}