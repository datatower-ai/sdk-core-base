use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Debug, PartialEq)]
pub(crate) enum MemValue {
    String(&'static str),
    Int8(i8),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
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

#[cfg(test)]
mod test {
    use crate::base::{mem, MemMap};
    use crate::base::MemValue::{Bool, Float32, Float64, Int32, Int64, Int8, String};

    #[test]
    fn it_works() {
        let mut m: &MemMap = mem();
        m.lock().unwrap().insert("key_1".to_string(), String("asd"));
        query_single_from_mem();

        insert_to_mem();
        query_from_mem();
    }

    fn query_single_from_mem() {
        let m = mem();
        println!("===> {:?}", m.lock().unwrap().get("key_1"));
    }

    fn insert_to_mem() {
        let mut m = mem().lock().unwrap();
        m.insert("key_i8".to_string(), Int8(1));
        m.insert("key_i32".to_string(), Int32(2));
        m.insert("key_i64".to_string(), Int64(3));
        m.insert("key_f32".to_string(), Float32(1.1));
        m.insert("key_f64".to_string(), Float64(1.2));
        m.insert("key_b".to_string(), Bool(false));
    }

    fn query_from_mem() {
        let m = mem().lock().unwrap();
        println!("===> key_i8: {:?}", m.get("key_i8"));
        println!("===> key_i32: {:?}", m.get("key_i32"));
        println!("===> key_i64: {:?}", m.get("key_i64"));
        println!("===> key_f32: {:?}", m.get("key_f32"));
        println!("===> key_f64: {:?}", m.get("key_f64"));
        println!("===> key_b: {:?}", m.get("key_b"));
    }
}