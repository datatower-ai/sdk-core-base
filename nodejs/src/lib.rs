use std::sync::atomic::Ordering;
use napi_derive::napi;
use serde_json::{Map, Value};
use common::util::error::DTError;
use common::util::result::{dissolve, dissolve_bool};

static VERSION: &'static str = "1.0.0";
static SDK_NAME: &'static str = "nodejs";

static TYPE_EVENT: &'static str = "track";
static TYPE_USER: &'static str = "user";

#[napi]
fn init(path: String, max_batch_len: u32, name_prefix: Option<String>, max_file_size_bytes: Option<u32>) -> bool {
    let mut config = Map::with_capacity(5);
    config.insert("consumer".to_string(), Value::from("log"));
    config.insert("path".to_string(), Value::from(path));
    config.insert("max_batch_len".to_string(), Value::from(max_batch_len));
    if let Some(name_prefix) = name_prefix {
        config.insert("name_prefix".to_string(), Value::from(name_prefix));
    }
    if let Some(max_file_size_bytes) = max_file_size_bytes {
        config.insert("max_file_size_bytes".to_string(), Value::from(max_file_size_bytes));
    }

    dissolve_bool::<(), DTError>(common::init_by_config(config)).unwrap_or(false)
}

#[napi]
fn track(dt_id: String, ac_id: String, event_name: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, event_name, TYPE_EVENT, properties)
}

#[napi]
fn user_set(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_set".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_set_once(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_set_once".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_add(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_add".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_unset(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_unset".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_delete(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_delete".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_append(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_append".to_string(), TYPE_USER, properties)
}

#[napi]
fn user_uniq_append(dt_id: String, ac_id: String, properties: Map<String, Value>) -> bool {
    add_event(dt_id, ac_id, "#user_uniq_append".to_string(), TYPE_USER, properties)
}

#[napi]
fn flush() -> () {
    dissolve::<(), DTError>(common::flush()).unwrap_or(())
}

#[napi]
fn close() -> () {
    dissolve::<(), DTError>(common::close()).unwrap_or(())
}

#[napi]
fn toggle_logger(enable: bool) -> () {
    common::util::logger::LOG_ENABLED.store(enable, Ordering::Relaxed);
    ()
}

fn add_event(dt_id: String, ac_id: String, event_name: String, event_type: &'static str, properties: Map<String, Value>) -> bool {
    let mut event = Map::with_capacity(properties.len() + 6);

    for (k, v) in properties {
        event.insert(k, v);
    }

    event.insert(String::from("#dt_id"), serde_json::Value::from(dt_id));
    event.insert(String::from("#acid"), serde_json::Value::from(ac_id));
    event.insert(String::from("#event_name"), serde_json::Value::from(event_name));
    event.insert(String::from("#event_type"), serde_json::Value::from(event_type));
    event.insert(String::from("#sdk_version_name"), serde_json::Value::from(VERSION));
    event.insert(String::from("#sdk_type"), serde_json::Value::from(SDK_NAME));

    dissolve_bool::<(), DTError>(common::add(event)).unwrap_or(false)
}