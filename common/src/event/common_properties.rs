use std::cell::Cell;
use std::sync::Mutex;
use serde_json::{Map, Value};
use crate::event::Event;
use crate::util::error::macros::{internal_error};
use crate::util::error::{DTError, Result};

pub(crate) type Props = Map<String, Value>;

static STATIC_COMMON_PROPS: Mutex<Cell<Option<Props>>> = Mutex::new(Cell::new(None));

// Currently not implemented with persist storage yet.
pub(crate) fn set_static_comm_props(props: Props) -> Result<()> {
    let Ok(scp) = STATIC_COMMON_PROPS.lock() else {
        return internal_error!("Failed to get lock for static_common_properties!");
    };
    scp.replace(Some(props));
    // store in local ...
    Ok(())
}

pub(crate) fn clear_static_comm_props() -> Result<()> {
    let Ok(scp) = STATIC_COMMON_PROPS.lock() else {
        return internal_error!("Failed to get lock for static_common_properties!");
    };
    scp.replace(None);
    // clear local stored.
    Ok(())
}

pub(crate) fn fulfill_by_comm_props(event: &mut Event) -> Result<()> {
    let Ok(mut scp) = STATIC_COMMON_PROPS.lock() else {
        return internal_error!("Failed to get lock for static_common_properties!");
    };
    let key_properties = String::from("properties");
    if let Some(scp) = scp.get_mut() {
        if let Some(Value::Object(properties)) = event.get_mut(&key_properties) {
            // insert to existed "properties"
            for (k, v) in scp {
                if !properties.contains_key(k) {
                    properties.insert(k.clone(), v.clone());
                }
            }
        } else {
            // create new if "properties" is not found
            let mut properties = Map::with_capacity(scp.len());
            for (k, v) in scp {
                if !properties.contains_key(k) {
                    properties.insert(k.clone(), v.clone());
                }
            }
            event.insert(key_properties, Value::Object(properties));
        }
    }
    Ok(())
}
