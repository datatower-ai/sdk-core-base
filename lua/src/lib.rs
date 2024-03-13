use mlua::prelude::*;
use mlua::{Table, Value};
use serde_json::Map;
use common::consumer::log::LogConsumer;

#[mlua::lua_module]
fn dt_core_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("init", lua.create_function(init)?)?;
    exports.set("add_event", lua.create_function(add_event)?)?;
    exports.set("verify_event", lua.create_function(verify_event)?)?;
    exports.set("flush", lua.create_function(flush)?)?;
    exports.set("close", lua.create_function(close)?)?;
    Ok(exports)
}

fn init(_: &Lua, table: Table) -> LuaResult<bool> {
    let Ok(path) = table.get("path") else {
        eprintln!("[DT Core] Failed to initialize: missing \"path\"!");
        return Ok(false);
    };

    let max_batch_len: mlua::Result<mlua::ffi::lua_Integer> = table.get("max_batch_len");
    let Ok(max_batch_len) = max_batch_len else {
        eprintln!("[DT Core] Failed to initialize: missing \"max_batch_len\"!");
        return Ok(false);
    };

    let name_prefix: Option<String> = if let Ok(name_prefix) = table.get("name_prefix") {
        Some(name_prefix)
    } else {
        None
    };
    let max_file_size_bytes: mlua::Result<mlua::ffi::lua_Integer> = table.get("max_file_size_bytes");
    let max_file_size_bytes: Option<u64> = if let Ok(max_file_size_bytes) = max_file_size_bytes {
        Some(max_file_size_bytes as u64)
    } else {
        None
    };

    let consumer = LogConsumer::new(
        path, max_batch_len as u32, name_prefix, max_file_size_bytes
    );
    Ok(common::init_consumer(consumer))
}

fn verify_event(_: &Lua, table: Table) -> LuaResult<bool> {
    let map: Map<String, serde_json::Value> = MyTable(table).into();
    Ok(common::util::data_verification::verify_event(&map))
}

fn add_event(_: &Lua, table: Table) -> LuaResult<bool> {
    let map: Map<String, serde_json::Value> = MyTable(table).into();
    Ok(common::add(map))
}

fn flush(_: &Lua, _: ()) -> LuaResult<()> {
    common::flush();
    Ok(())
}

fn close(_: &Lua, _: ()) -> LuaResult<()> {
    common::close();
    Ok(())
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
                    eprintln!("[DT Core] Such value is unsupported: {:?}", value);
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
                eprintln!("[DT Core] Given value is not support, {:?}", self.0);
                None
            }
        }
    }
}
