use std::collections::HashMap;
use regex::Regex;
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use crate::event::Event;
use crate::util::error::{DTError, Result};
use crate::util::error::macros::verify_error;

const NAME_REGEX_STR: &'static str = r"^[a-zA-Z#][a-zA-Z\d_#]{0,49}$";
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

pub(crate) const META_PROPS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#app_id", TypeConstraint::String), ("#bundle_id", TypeConstraint::String),
    ("#android_id", TypeConstraint::String), ("#gaid", TypeConstraint::String),
    ("#dt_id", TypeConstraint::String), ("#acid", TypeConstraint::String),
    ("#event_name", TypeConstraint::String), ("#event_type", TypeConstraint::String),
    ("#event_time", TypeConstraint::Integer), ("#event_syn", TypeConstraint::String),
    ("properties", TypeConstraint::Object), ("#debug", TypeConstraint::Bool),
]));
const COMPULSORY_META_PROPS: Lazy<Vec<String>> = Lazy::new(|| vec!(
    String::from("#app_id"), String::from("#bundle_id"),
    String::from("#dt_id"), String::from("#event_time"),
    String::from("#event_name"), String::from("#event_type"),
    String::from("#event_syn"), String::from("properties")
));

const PRESET_PROPS_COMMON: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#bundle_id", TypeConstraint::String), ("#zone_offset", TypeConstraint::Integer),
    ("#session_id", TypeConstraint::String), ("#device_manufacturer", TypeConstraint::String),
    ("#is_foreground", TypeConstraint::Bool), ("#mcc", TypeConstraint::String),
    ("#mnc", TypeConstraint::String), ("#os_country_code", TypeConstraint::String),
    ("#os_lang_code", TypeConstraint::String), ("#app_version_code", TypeConstraint::Integer),
    ("#app_version_name", TypeConstraint::String), ("#sdk_type", TypeConstraint::String),
    ("#sdk_version_name", TypeConstraint::String), ("#os", TypeConstraint::String),
    ("#os_version_name", TypeConstraint::String), ("#os_version_code", TypeConstraint::Number),
    ("#device_brand", TypeConstraint::String), ("#device_model", TypeConstraint::String),
    ("#screen_height", TypeConstraint::Number), ("#screen_width", TypeConstraint::Number),
    ("#memory_used", TypeConstraint::String), ("#storage_used", TypeConstraint::String),
    ("#network_type", TypeConstraint::String), ("#simulator", TypeConstraint::Bool),
    ("#fps", TypeConstraint::Number), ("#scene", TypeConstraint::String),
    ("#mp_platform", TypeConstraint::String)
]));
const PRESET_PROPS_USER_COMMON: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#active_device_model", TypeConstraint::String), ("#active_network_type", TypeConstraint::String),
    ("#active_os_version_name", TypeConstraint::String), ("#active_os", TypeConstraint::String),
    ("#active_os_lang_code", TypeConstraint::String), ("#firebase_iid", TypeConstraint::String),
]));
const PRESET_PROPS_AD: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_seq", TypeConstraint::String), ("#ad_id", TypeConstraint::String),
    ("#ad_type_code", TypeConstraint::Integer), ("#ad_platform_code", TypeConstraint::Integer),
    ("#ad_entrance", TypeConstraint::String), ("#ad_result", TypeConstraint::Bool),
    ("#ad_duration", TypeConstraint::Integer), ("#ad_location", TypeConstraint::String),
    ("#ad_value", TypeConstraint::Number), ("#ad_currency", TypeConstraint::String),
    ("#ad_precision", TypeConstraint::String), ("#ad_country_code", TypeConstraint::String),
    ("#ad_mediation_code", TypeConstraint::Integer), ("#ad_mediation_id", TypeConstraint::String),
    ("#ad_conversion_source", TypeConstraint::String), ("#ad_click_gap", TypeConstraint::String),
    ("#ad_return_gap", TypeConstraint::String), ("#error_code", TypeConstraint::Integer),
    ("#error_message", TypeConstraint::String), ("#load_result", TypeConstraint::String),
    ("#load_duration", TypeConstraint::String)
]));
const PRESET_PROPS_IAS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ias_original_order", TypeConstraint::String), ("#ias_order", TypeConstraint::String),
    ("#ias_sku", TypeConstraint::String), ("#ias_price", TypeConstraint::Number),
    ("#ias_currency", TypeConstraint::String)
]));
const PRESET_PROPS_APP_INSTALL: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#referrer_url", TypeConstraint::String), ("#referrer_click_time", TypeConstraint::Integer),
    ("#app_install_time", TypeConstraint::Integer), ("#instant_experience_launched", TypeConstraint::Bool),
    ("#failed_reason", TypeConstraint::String), ("#cnl", TypeConstraint::String)
]));
const PRESET_PROPS_SESSION_START: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#is_first_time", TypeConstraint::Bool), ("#resume_from_background", TypeConstraint::Bool),
    ("#start_reason", TypeConstraint::String), ("#background_duration", TypeConstraint::Integer)
]));
const PRESET_PROPS_SESSION_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#session_duration", TypeConstraint::Integer)
]));
const PRESET_PROPS_IAP_PURCHASE_SUCCESS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#iap_order", TypeConstraint::String), ("#iap_sku", TypeConstraint::String),
    ("#iap_price", TypeConstraint::Number), ("#iap_currency", TypeConstraint::String)
]));

const EMPTY_PROPS_LIST: PropsConstraintMap = Lazy::new(|| HashMap::new());
const USER_PROPS_LIST_TUPLE: (PropsConstraintMap, PropsConstraintMap) = (PRESET_PROPS_USER_COMMON, EMPTY_PROPS_LIST);
const EMPTY_PROPS_LIST_TUPLE: (PropsConstraintMap, PropsConstraintMap) = (PRESET_PROPS_USER_COMMON, EMPTY_PROPS_LIST);
// { Event name: (Shared properties, Extra event-specific properties) }
const PRESET_EVENTS: Lazy<HashMap<&str, (PropsConstraintMap, PropsConstraintMap)>> = Lazy::new(|| HashMap::from([
    ("#app_install", (PRESET_PROPS_APP_INSTALL, EMPTY_PROPS_LIST)),
    ("#session_start", (PRESET_PROPS_SESSION_START, EMPTY_PROPS_LIST)),
    ("#session_end", (PRESET_PROPS_SESSION_END, EMPTY_PROPS_LIST)),
    ("#ad_load_begin", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_load_end", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_to_show", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_show", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_show_failed", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_close", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_click", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_left_app", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_return_app", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_rewarded", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_conversion", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#ad_paid", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#iap_purchase_success", (PRESET_PROPS_IAP_PURCHASE_SUCCESS, EMPTY_PROPS_LIST)),
    ("#ias_subscribe_success", (PRESET_PROPS_IAS, EMPTY_PROPS_LIST)),
    ("#ias_subscribe_notify", (PRESET_PROPS_IAS, EMPTY_PROPS_LIST)),
]));

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
            if let Some(props_tuple) = PRESET_EVENTS.get(event_name.as_str()) {
                verify_preset_event(event_name, properties, props_tuple)
            } else {
                return verify_error!("event_name (\"{}\") is out of scope (preset)!", event_name);
            }
        } else {
            verify_custom_properties(event_name, properties)
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
    props_tuple: &(PropsConstraintMap, PropsConstraintMap)
) -> Result<()> {
    for (key, value) in properties {
        verify_properties(event_name, key, value, props_tuple)?
    }
    Ok(())
}

fn verify_properties(
    event_name: &String,
    key: &String, value: &Value,
    props_tuple: &(PropsConstraintMap, PropsConstraintMap)
) -> Result<()> {
    if !NAME_RE.is_match(key) {
        return verify_error!("Property name (\"{}\") is invalid!", key);
    }

    if is_preset(key) {
        if let Some(constraint) = find_constraint_in_preset_event(key.as_str(), props_tuple, &PRESET_PROPS_COMMON) {
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

fn find_constraint_in_preset_event<'a>(
    prop_name: &str,
    (props1, props2): &'a (PropsConstraintMap, PropsConstraintMap),
    common_pcm: &'a PropsConstraintMap
) -> Option<&'a TypeConstraint> {
    common_pcm.get(prop_name).or(props1.get(prop_name).or(props2.get(prop_name)))
}

fn verify_user_event(event_name: &String, properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        verify_properties(event_name, k, v, &USER_PROPS_LIST_TUPLE)?
    }

    if event_name == "#user_append" || event_name == "#user_uniq_append" {
        verify_all_custom_props_are_list(properties)
    } else if event_name == "#user_add" {
        verify_all_custom_props_are_num(properties)
    } else {
        Ok(())
    }
}

fn verify_custom_properties(event_name: &String, properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        verify_properties(event_name, k, v, &EMPTY_PROPS_LIST_TUPLE)?
    }
    Ok(())
}

fn verify_all_custom_props_are_list(properties: &Map<String, Value>) -> Result<()> {
    for (k, v) in properties {
        if is_preset(k) {
            return Ok(());
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
            return Ok(());
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
        let pass = verify_event(obj.as_object().unwrap());
        assert_eq!(pass.is_ok(), target)
    }

    #[test]
    fn its_work() {
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
    }
}