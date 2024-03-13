use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::consumer::Consumer;

pub(crate) enum MemValue {
    // String(&'static str),
    // Int8(i8),
    // Int32(i32),
    // Int64(i64),
    // Float32(f32),
    // Float64(f64),
    // Bool(bool),
    Consumer(Box<dyn Consumer>),
}

unsafe impl Send for MemValue {}
unsafe impl Sync for MemValue {}

pub(crate) type MemMap = Mutex<HashMap<String, MemValue>>;

pub(crate) fn mem() -> &'static MemMap {
    static MEM: OnceLock<MemMap> = OnceLock::new();
    MEM.get_or_init(|| {
        Mutex::new(HashMap::new())
    })
}