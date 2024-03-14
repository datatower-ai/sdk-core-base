#[allow(unused_imports)]
pub static mut LOG_ENABLED: bool = false;
pub static mut LOG_LEVEL: i8 = 0;


pub mod logger {
    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)*) => {
            unsafe {
                if ($crate::util::logger::LOG_ENABLED && 0 >= $crate::util::logger::LOG_LEVEL) {
                    println!("DEBUG: [DT Core] {}", format!($($arg)*));
                }
            }
        };
    }

    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)*) => {
            unsafe {
                if ($crate::util::logger::LOG_ENABLED && 1 >= $crate::util::logger::LOG_LEVEL) {
                    println!("INFO: [DT Core] {}", format!($($arg)*));
                }
            }
        };
    }

    #[macro_export]
    macro_rules! log_warning {
        ($($arg:tt)*) => {
            unsafe {
                if ($crate::util::logger::LOG_ENABLED && 2 >= $crate::util::logger::LOG_LEVEL) {
                    println!("WARNING: [DT Core] {}",  format!($($arg)*));
                }
            }
        };
    }

    #[macro_export]
    macro_rules! log_error {
        ($($arg:tt)*) => {
            unsafe {
                if (3 >= $crate::util::logger::LOG_LEVEL) {
                    eprintln!("ERROR: [DT Core] {}",  format!($($arg)*));
                }
            }
        };
    }

    pub use log_info;
    pub use log_warning;
    pub use log_error;
}