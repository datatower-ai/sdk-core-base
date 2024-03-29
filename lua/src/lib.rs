use std::sync::atomic::Ordering;
use mlua::prelude::*;
use mlua::{Table, Value};
use serde_json::Map;
use common::log_error;
use common::util::result::{dissolve, dissolve_bool};

#[mlua::lua_module]
fn dt_core_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("init", lua.create_function(init)?)?;
    exports.set("add_event", lua.create_function(add_event)?)?;
    exports.set("flush", lua.create_function(flush)?)?;
    exports.set("close", lua.create_function(close)?)?;
    exports.set("enable_log", lua.create_function(toggle_logger)?)?;
    Ok(exports)
}

fn init(_: &Lua, table: Table) -> LuaResult<bool> {
    dissolve_bool(common::init_by_config(MyTable(table).into()))
}

fn add_event(_: &Lua, table: Table) -> LuaResult<bool> {
    let map: Map<String, serde_json::Value> = MyTable(table).into();
    dissolve_bool(common::add(map))
}

fn flush(_: &Lua, _: ()) -> LuaResult<()> {
    dissolve(common::flush())
}

fn close(_: &Lua, _: ()) -> LuaResult<()> {
    dissolve(common::close())
}

fn toggle_logger(_: &Lua, enable: bool) -> LuaResult<()> {
    common::util::logger::LOG_ENABLED.store(enable, Ordering::Relaxed);
    Ok(())
}

fn set_static_common_properties(_: &Lua, table: Table) -> LuaResult<()> {
    let map: Map<String, serde_json::Value> = MyTable(table).into();
    dissolve(common::set_static_common_props(map))
}

fn clear_static_common_properties(_: &Lua) -> LuaResult<()> {
    dissolve(common::clear_static_common_props())
}

struct MyTable<'a>(Table<'a>);

impl Into<Map<String, serde_json::Value>> for MyTable<'_> {
    fn into(self) -> Map<String, serde_json::Value> {
        let mut result = Map::new();
        for pair in self.0.pairs::<mlua::String, Value>() {
            if let Ok((key, value)) = pair {
                let sjv: Option<serde_json::Value> = MyValue(value.clone()).into();

                if let Some(sjv) = sjv {
                    result.insert(key.to_str().unwrap().to_string(), sjv);
                } else {
                    log_error!("Such value is unsupported: {:?}", value);
                }
            }
        }
        result
    }
}

struct MyValue<'a>(Value<'a>);

impl Into<Option<serde_json::Value>> for MyValue<'_> {
    fn into(self) -> Option<serde_json::Value> {
        match self.0 {
            Value::String(s) => Some(serde_json::Value::from(s.to_str().unwrap())),
            Value::Integer(i) => Some(serde_json::Value::from(i)),
            Value::Number(n) => Some(serde_json::Value::from(n)),
            Value::Boolean(b) => Some(serde_json::Value::from(b)),
            Value::Table(t) => {
                let mut has_non_int = false;
                let mut has_non_str = false;

                let pairs = t.pairs::<Value, Value>();
                let mut kv_pair: Vec<(Value, Value)> = Vec::new();

                for pair in pairs {
                    if let Ok(pair) = pair {
                        if let (Value::Integer(_), _) = pair {
                            has_non_str = true;
                            if has_non_int {
                                break;
                            }
                        } else if let (Value::String(_), _) = pair {
                            has_non_int = true;
                            if has_non_str {
                                break;
                            }
                        } else {
                            has_non_int = true;
                            has_non_str = true;
                            break;
                        }
                        kv_pair.push(pair)
                    }
                }

                if !has_non_int {
                    // array
                    let result: Vec<serde_json::Value> = kv_pair.into_iter().map_while(|(_, v)| {
                        MyValue(v).into()
                    }).collect();
                    Some(serde_json::Value::from(result))
                } else if !has_non_str {
                    // Map
                    let result: Map<String, serde_json::Value> = kv_pair.into_iter().map_while(|(k, v)| {
                        if let Value::String(s) = k {
                            let new_value: Option<serde_json::Value> = MyValue(v).into();
                            if let Some(new_value) = new_value {
                                Some((s.to_str().unwrap().to_string(), new_value))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }).collect();
                    Some(serde_json::Value::from(result))
                } else {
                    None
                }
            },
            _ => {
                log_error!("Given value is not supported, {:?}", self.0);
                None
            }
        }
    }
}
