use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = c_str.to_str().unwrap_or_else(|_| "there");

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_ai_datatower_RustGreetings_greeting(mut env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        let world = rust_greeting(env.get_string(&java_pattern).expect("invalid pattern string").as_ptr());
        let world_ptr = CString::from_raw(world);
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
