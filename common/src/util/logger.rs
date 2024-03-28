use std::sync::atomic::{AtomicBool, AtomicI8};

#[allow(unused_imports)]
pub static LOG_ENABLED: AtomicBool = AtomicBool::new(false);
pub static LOG_LEVEL: AtomicI8 = AtomicI8::new(0);

pub mod logger {
    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)*) => {
            if ($crate::util::logger::LOG_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
                && 0 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("DEBUG: [DT Core] {}", format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)*) => {
            if ($crate::util::logger::LOG_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
                && 1 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("INFO: [DT Core] {}", format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_warning {
        ($($arg:tt)*) => {
            if ($crate::util::logger::LOG_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
                && 2 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("WARNING: [DT Core] {}",  format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_error {
        ($($arg:tt)*) => {
            if (3 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                eprintln!("ERROR: [DT Core] {}",  format!($($arg)*));
            }
        };
    }

    pub use log_info;
    pub use log_warning;
    pub use log_error;
}