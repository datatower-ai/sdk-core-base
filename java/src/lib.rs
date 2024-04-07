use Ordering::Relaxed;
use std::sync::atomic::Ordering;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jboolean;
use serde_json::Value;
use common::log_error;
use common::util::error::DTError;
use common::util::result::{dissolve, dissolve_bool};
use crate::parser::jmap2map;

type JniError = jni::errors::Error;
type JniResult<T> = Result<T, JniError>;

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_init<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, config: JObject<'local>) -> jboolean {
    let raw_config = env.get_map(&config).expect("Couldn't get config map");
    let Ok(config) = jmap2map(&mut env, raw_config) else {
        log_error!("Failed to parse config map");
        return jboolean::from(false);
    };
    let result = dissolve_bool::<(), DTError>(
        common::init_by_config(config)
    ).unwrap_or(false);
    jboolean::from(result)
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_addEvent<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, event: JObject<'local>) -> jboolean {
    let event = env.get_map(&event).expect("Couldn't get event/properties");
    let Ok(event) = jmap2map(&mut env, event) else {
        log_error!("Failed to parse event/properties");
        return jboolean::from(false);
    };
    let result = dissolve_bool::<(), DTError>(
        common::add(event)
    ).unwrap_or(false);
    jboolean::from(result)
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_addEventStr<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, event: JString<'local>) -> jboolean {
    let Ok(event) = env.get_string(&event) else {
        log_error!("Failed to get event");
        return jboolean::from(false);
    };
    let event_str: String = event.into();
    let event = match serde_json::from_str::<Value>(event_str.as_str()) {
        Ok(json) => match json {
            Value::Object(map) => map,
            _ => {
                log_error!("Failed to parse init config! Given: {json:?}");
                return jboolean::from(false);
            },
        },
        Err(err) => {
            log_error!("Failed to parse event, {err}");
            return jboolean::from(false);
        }
    };

    let result = dissolve_bool::<(), DTError>(
        common::add(event)
    ).unwrap_or(false);
    jboolean::from(result)
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_flush<'local>(_env: JNIEnv<'local>, _class: JClass<'local>) {
    dissolve::<(), DTError>(common::flush()).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_close<'local>(_env: JNIEnv<'local>, _class: JClass<'local>) {
    dissolve::<(), DTError>(common::close()).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_toggleLogger<'local>(_env: JNIEnv<'local>, _class: JClass<'local>, enable: jboolean) {
    common::util::logger::LOG_ENABLED.store(enable != 0, Relaxed);
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_setStaticCommonProperties<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, properties: JObject<'local>) {
    let event = env.get_map(&properties).expect("Couldn't get event/properties");
    let Ok(properties) = jmap2map(&mut env, event) else {
        log_error!("Failed to parse event/properties");
        return;
    };
    dissolve::<(), DTError>(common::set_static_common_props(properties)).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_ai_datatower_sdk_DTBase_clearStaticCommonProperties<'local>(_env: JNIEnv<'local>, _class: JClass<'local>) {
    dissolve::<(), DTError>(common::clear_static_common_props()).unwrap();
}

mod parser {
    use serde_json::{Map, Value};
    use jni::JNIEnv;
    use jni::objects::{JList, JMap, JObject, JString};
    use common::log_error;
    use super::JniResult;

    static CLASS_INTEGER: &str = "java/lang/Integer";
    static CLASS_STRING: &str = "java/lang/String";
    static CLASS_LONG: &str = "java/lang/Long";
    static CLASS_SHORT: &str = "java/lang/Short";
    static CLASS_BYTE: &str = "java/lang/Byte";
    static CLASS_FLOAT: &str = "java/lang/Float";
    static CLASS_DOUBLE: &str = "java/lang/Double";
    static CLASS_CHAR: &str = "java/lang/Character";
    static CLASS_BOOLEAN: &str = "java/lang/Boolean";
    static CLASS_MAP: &str = "java/util/Map";
    static CLASS_LIST: &str = "java/util/List";

    pub(super) fn jmap2map<'local>(env: &mut JNIEnv<'local>, jmap: JMap) -> JniResult<Map<String, Value>> {
        let mut map = Map::new();
        let mut iterator = jmap.iter(env).unwrap();
        while let Some((raw_key, raw_value)) = iterator.next(env)? {
            let js_key = JString::from(raw_key);
            let key: String = env.get_string(&js_key)?.into();
            let value = jobject2value(env, raw_value, &key)?;
            match value {
                Some(value) => { map.insert(key, value); },
                None => {}
            }
            env.delete_local_ref(js_key)?;
        }
        Ok(map)
    }

    fn jlist2value<'local>(env: &mut JNIEnv<'local>, jlist: JList) -> JniResult<Value> {
        let mut list: Vec<Value> = Vec::new();
        let mut iterator = jlist.iter(env)?;
        let tmp = String::new();
        while let Some(item) = iterator.next(env)? {
            let value = jobject2value(env, item, &tmp)?;

            match value {
                Some(value) => list.push(value),
                None => {}
            }
        }
        Ok(Value::from(list))
    }

    fn jobject2value<'local>(env: &mut JNIEnv<'local>, jobject: JObject, key: &String) -> JniResult<Option<Value>> {
        // remember to delete_local_ref()!
        if jobject.is_null() {
            Ok(Some(Value::Null))
        } else if env.is_instance_of(&jobject, CLASS_INTEGER)? {
            let int = env.call_method(&jobject, "intValue", "()I", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(int.i()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_STRING)? {
            let js_string = JString::from(jobject);
            let value_string: String = env.get_string(&js_string)?.into();
            env.delete_local_ref(js_string)?;
            Ok(Some(Value::from(value_string)))
        } else if env.is_instance_of(&jobject, CLASS_LONG)? {
            let long = env.call_method(&jobject, "longValue", "()J", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(long.j()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_SHORT)? {
            let value = env.call_method(&jobject, "shortValue", "()S", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(value.s()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_BYTE)? {
            let value = env.call_method(&jobject, "byteValue", "()B", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(value.b()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_FLOAT)? {
            let value = env.call_method(&jobject, "floatValue", "()F", &[])?;
            let float: f64 = value.f()?.to_owned() as f64;
            let float: f64 = (float * 10_000_000_f64).floor() / 10_000_000_f64;     // 7 digits precision.
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(float)))
        } else if env.is_instance_of(&jobject, CLASS_DOUBLE)? {
            let value = env.call_method(&jobject, "doubleValue", "()D", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(value.d()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_CHAR)? {
            let value = env.call_method(&jobject, "charValue", "()C", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(value.c()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_BOOLEAN)? {
            let value = env.call_method(&jobject, "booleanValue", "()Z", &[])?;
            env.delete_local_ref(jobject)?;
            Ok(Some(Value::from(value.z()?.to_owned())))
        } else if env.is_instance_of(&jobject, CLASS_MAP)? {
            let jmap = env.get_map(&jobject)?;
            let map = jmap2map(env, jmap)?;
            Ok(Some(Value::from(map)))
        } else if env.is_instance_of(&jobject, CLASS_LIST)? {
            let jlist = env.get_list(&jobject)?;
            let arr = jlist2value(env, jlist)?;
            Ok(Some(Value::from(arr)))
        } else {
            // might be array type.
            log_error!("Only accepting primitives, List and Map as value type! (note: currently Array is not supported)");
            if !key.is_empty() {
                log_error!("The key \"{key}\" and associated value will be ignored!")
            }
            env.delete_local_ref(jobject)?;
            Ok(None)
        }
    }
}