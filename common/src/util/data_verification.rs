use std::collections::HashMap;
use regex::Regex;
use once_cell::unsync::Lazy;
use serde_json::{Map, Value};

const NAME_REGEX_STR: &'static str = r"^[#$a-zA-Z][a-zA-Z0-9_]{0,63}$";
const NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(NAME_REGEX_STR).unwrap());

#[derive(Debug, PartialEq, Copy, Clone)]
enum TypeConstraint {
    String,
    Number,             // Integer + Float
    Integer,
    Float,
    Bool,
    Object,
    Array,
}

type PropsConstraintMap = Lazy<HashMap<&'static str, TypeConstraint>>;

const META_PROPS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#app_id", TypeConstraint::String), ("#bundle_id", TypeConstraint::String),
    ("#android_id", TypeConstraint::String), ("#gaid", TypeConstraint::String),
    ("#dt_id", TypeConstraint::String), ("#acid", TypeConstraint::String),
    ("#event_time", TypeConstraint::Integer), ("#event_syn", TypeConstraint::String),
    ("properties", TypeConstraint::Object)
]));
const COMPULSORY_META_PROPS: Lazy<Vec<String>> = Lazy::new(|| vec!(
    String::from("#app_id"), String::from("#bundle_id"),
    String::from("#event_time"), String::from("#event_name"),
    String::from("#event_type"), String::from("#event_syn"),
    String::from("properties")
));

const PRESET_PROPS_COMMON: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("$uid", TypeConstraint::String), ("#dt_id", TypeConstraint::String), ("#acid", TypeConstraint::String),
    ("#event_syn", TypeConstraint::String), ("#session_id", TypeConstraint::String),
    ("#device_manufacturer", TypeConstraint::String), ("#event_name", TypeConstraint::String),
    ("#is_foreground", TypeConstraint::Bool), ("#android_id", TypeConstraint::String),
    ("#gaid", TypeConstraint::String), ("#mcc", TypeConstraint::String), ("#mnc", TypeConstraint::String),
    ("#os_country_code", TypeConstraint::String), ("#os_lang_code", TypeConstraint::String),
    ("#event_time", TypeConstraint::Integer), ("#bundle_id", TypeConstraint::String),
    ("#app_version_code", TypeConstraint::Integer), ("#app_version_name", TypeConstraint::String),
    ("#sdk_type", TypeConstraint::String), ("#sdk_version_name", TypeConstraint::String),
    ("#os", TypeConstraint::String), ("#os_version_name", TypeConstraint::String),
    ("#os_version_code", TypeConstraint::Integer), ("#device_brand", TypeConstraint::String),
    ("#device_model", TypeConstraint::String), ("#build_device", TypeConstraint::String),
    ("#screen_height", TypeConstraint::Integer), ("#screen_width", TypeConstraint::Integer),
    ("#memory_used", TypeConstraint::String), ("#storage_used", TypeConstraint::String),
    ("#network_type", TypeConstraint::String), ("#simulator", TypeConstraint::Bool),
    ("#fps", TypeConstraint::Integer), ("$ip", TypeConstraint::String), ("$country_code", TypeConstraint::String),
    ("$server_time", TypeConstraint::Integer)
]));
const PRESET_PROPS_AD: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ad_seq", TypeConstraint::String), ("#ad_id", TypeConstraint::String),
    ("#ad_type_code", TypeConstraint::String), ("#ad_platform_code", TypeConstraint::String),
    ("#ad_entrance", TypeConstraint::String), ("#ad_result", TypeConstraint::Bool),
    ("#ad_duration", TypeConstraint::Integer), ("#ad_location", TypeConstraint::String),
    ("#errorCode", TypeConstraint::Integer), ("#errorMessage", TypeConstraint::String),
    ("#ad_value", TypeConstraint::String), ("#ad_currency", TypeConstraint::String),
    ("#ad_precision", TypeConstraint::String), ("#ad_country_code", TypeConstraint::String),
    ("#ad_mediation_code", TypeConstraint::String), ("#ad_mediation_id", TypeConstraint::String),
    ("#ad_conversion_source", TypeConstraint::String), ("#ad_click_gap", TypeConstraint::String),
    ("#ad_return_gap", TypeConstraint::String), ("#error_code", TypeConstraint::String),
    ("#error_message", TypeConstraint::String), ("#load_result", TypeConstraint::String),
    ("#load_duration", TypeConstraint::String)
]));
const PRESET_PROPS_IAS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#ias_seq", TypeConstraint::String), ("#ias_original_order", TypeConstraint::String),
    ("#ias_order", TypeConstraint::String), ("#ias_sku", TypeConstraint::String),
    ("#ias_price", TypeConstraint::Float), ("#ias_currency", TypeConstraint::String),
    ("$ias_price_exchange", TypeConstraint::Float)
]));
const PRESET_PROPS_APP_INSTALL: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#referrer_url", TypeConstraint::String), ("#referrer_click_time", TypeConstraint::Integer),
    ("#app_install_time", TypeConstraint::Integer), ("#instant_experience_launched", TypeConstraint::Bool),
    ("#failed_reason", TypeConstraint::String), ("#cnl", TypeConstraint::String)
]));
const PRESET_PROPS_SESSION_START: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#is_first_time", TypeConstraint::Bool), ("#resume_from_background", TypeConstraint::Bool),
    ("#start_reason", TypeConstraint::String)
]));
const PRESET_PROPS_D_APP_INSTALL: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("$network_id", TypeConstraint::String), ("$network_name", TypeConstraint::String),
    ("$tracker_id", TypeConstraint::String), ("$tracker_name", TypeConstraint::String),
    ("$channel_id", TypeConstraint::String), ("$channel_sub_id", TypeConstraint::String),
    ("$channel_ssub_id", TypeConstraint::String), ("$channel_name", TypeConstraint::String),
    ("$channel_sub_name", TypeConstraint::String), ("$channel_ssub_name", TypeConstraint::String),
    ("$channel_platform_id", TypeConstraint::Integer), ("$channel_platform_name", TypeConstraint::String),
    ("$attribution_source", TypeConstraint::String), ("$fraud_network_id", TypeConstraint::String),
    ("$original_tracker_id", TypeConstraint::String), ("$original_tracker_name", TypeConstraint::String),
    ("$original_network_id", TypeConstraint::String), ("$original_network_name", TypeConstraint::String)
]));
const PRESET_PROPS_SESSION_END: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#session_duration", TypeConstraint::Integer)
]));
const PRESET_PROPS_D_AD_CONVERSION: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("$earnings", TypeConstraint::Float)
]));
const PRESET_PROPS_IAP_PURCHASE_SUCCESS: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("#iap_order", TypeConstraint::String), ("#iap_sku", TypeConstraint::String),
    ("#iap_price", TypeConstraint::Float), ("#iap_currency", TypeConstraint::String),
    ("$iap_price_exchange", TypeConstraint::Float)
]));
const PRESET_PROPS_D_IAS_SUBSCRIBE_NOTIFY: PropsConstraintMap = Lazy::new(|| HashMap::from([
    ("$original_ios_notification_type", TypeConstraint::String)
]));

const EMPTY_PROPS_LIST: PropsConstraintMap = Lazy::new(|| HashMap::new());
const PRESET_EVENTS: Lazy<HashMap<&str, (PropsConstraintMap, PropsConstraintMap)>> = Lazy::new(|| HashMap::from([
    ("#app_install", (PRESET_PROPS_APP_INSTALL, EMPTY_PROPS_LIST)),
    ("#session_start", (PRESET_PROPS_SESSION_START, EMPTY_PROPS_LIST)),
    ("$app_install", (PRESET_PROPS_D_APP_INSTALL, EMPTY_PROPS_LIST)),
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
    ("$ad_conversion", (PRESET_PROPS_AD, PRESET_PROPS_D_AD_CONVERSION)),
    ("#ad_paid", (PRESET_PROPS_AD, EMPTY_PROPS_LIST)),
    ("#iap_purchase_success", (PRESET_PROPS_IAP_PURCHASE_SUCCESS, EMPTY_PROPS_LIST)),
    ("#ias_subscribe_success", (PRESET_PROPS_IAS, EMPTY_PROPS_LIST)),
    ("#ias_subscribe_notify", (PRESET_PROPS_IAS, EMPTY_PROPS_LIST)),
    ("$ias_subscribe_notify", (PRESET_PROPS_IAS, PRESET_PROPS_D_IAS_SUBSCRIBE_NOTIFY)),
]));

pub fn verify_event(event_map: &Map<String, Value>) -> bool {
    for prop in COMPULSORY_META_PROPS.iter() {
        if let Some(value) = event_map.get(prop) {
            if let Some(constraint) = META_PROPS.get(prop.as_str()) {
                if !check_type_constraint(value, constraint) {
                    println!("[DT Core] Type of value of meta property is not valid! Expected: {:?}, got: {}", constraint, value);
                    return false;
                }
            }
        } else {
            println!("[DT Core] Meta property \"{}\" is required, but missing!", prop);
            return false;
        }
    }

    if !check_meta_is_string_and_nonempty(event_map, String::from("#app_id")) {
        return false;
    }

    if !check_meta_is_string_and_nonempty(event_map, String::from("#dt_id")) {
        return false;
    }

    let Some(Value::String(event_name)) = event_map.get("#event_name") else {
        println!("[DT Core] #event_name is missing or it's type is invalid!");
        return false;
    };

    if !NAME_RE.is_match(event_name) {
        println!("[DT Core] #event_name must be a valid variable name!");
        return false;
    }

    let Some(Value::String(event_type)) = event_map.get("#event_type") else {
        println!("[DT Core] #event_type is missing or it's type is invalid!");
        return false;
    };

    let Some(Value::Object(properties)) = event_map.get("properties") else {
        println!("[DT Core] properties is missing or it's type is invalid!");
        return false;
    };

    if event_type == "track" && (event_name.starts_with("#") || event_name.starts_with("$")) {
        if let Some(props_tuple) = PRESET_EVENTS.get(event_name.as_str()) {
            verify_preset_properties(event_name, properties, props_tuple)
        } else {
            println!("[DT Core] event_name (\"{}\") is out of scope!", event_name);
            false
        }
    } else {
        verify_custom_properties(event_name, properties)
    }
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

fn check_meta_is_string_and_nonempty(event_map: &Map<String, Value>, key: String) -> bool {
    if let Some(value) = event_map.get(&key) {
        if let Value::String(value) = value {
            if value.len() == 0 {
                println!("[DT Core] {} cannot be empty!", key);
                false
            } else {
                true
            }
        } else {
            println!("[DT Core] {} should be a string!", key);
            false
        }
    } else {
        println!("[DT Core] {} is required, but missing", key);
        false
    }
}

fn verify_preset_properties(
    event_name: &String,
    properties: &Map<String, Value>,
    props_tuple: &(PropsConstraintMap, PropsConstraintMap)
) -> bool {
    for (key, value) in properties {
        if let Some(constraint) = find_constraint_in_preset_event(key.as_str(), props_tuple, &PRESET_PROPS_COMMON) {
            if !check_type_constraint(value, constraint) {
                println!(
                    "The type of value for property \"{}\" is not valid (Given: {}, Expected: {:?})!",
                    key, value, constraint
                );
                return false;
            }
        } else {
            println!(
                "key of property (\"{}\") is out of scope for preset event (\"{}\")!",
                key, event_name
            );
            return false
        }
    }
    true
}

fn find_constraint_in_preset_event<'a>(
    prop_name: &str,
    (props1, props2): &'a (PropsConstraintMap, PropsConstraintMap),
    common_pcm: &'a PropsConstraintMap
) -> Option<&'a TypeConstraint> {
    common_pcm.get(prop_name).or(props1.get(prop_name).or(props2.get(prop_name)))
}

fn verify_custom_properties(event_name: &String, properties: &Map<String, Value>) -> bool {
    if event_name == "#user_append" || event_name == "#user_uniq_append" {
        verify_sp_event_4_list(properties)
    } else if event_name == "#user_add" {
        verify_sp_event_4_num(properties)
    } else {
        for (k, _) in properties {
            if !verify_prop_name(k) {
                return false;
            }
        }
        true
    }
}

fn verify_prop_name(name: &String) -> bool {
    if !NAME_RE.is_match(name) {
        println!("[DT Core] Property name (\"{}\") is invalid!", name);
        false
    } else {
        true
    }
}

fn verify_sp_event_4_list(properties: &Map<String, Value>) -> bool {
    for (k, v) in properties {
        if !verify_prop_name(k) {
            return false;
        }
        let Value::Array(_) = v else {
            println!("[DT Core] Type of value in this event should be List");
            return false;
        };
    }
    true
}

fn verify_sp_event_4_num(properties: &Map<String, Value>) -> bool {
    for (k, v) in properties {
        if !verify_prop_name(k) {
            return false;
        }
        let Value::Number(_) = v else {
            println!("[DT Core] Type of value in this event should be Number");
            return false;
        };
    }
    true
}