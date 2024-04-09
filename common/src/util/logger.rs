use std::sync::atomic::{AtomicBool, AtomicI8};

#[allow(unused_imports)]
pub static LOG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Log level
/// 0: Debug
/// 1: Info
/// 2: Warning
/// 3: Error
pub static LOG_LEVEL: AtomicI8 = AtomicI8::new(0);

pub mod logger {
    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)*) => {
            if ($crate::util::logger::LOG_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
                && 0 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("[DT Core | DEBUG | {}] {}", $crate::util::datetime::get_fmt_datetime(), format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)*) => {
            if ($crate::util::logger::LOG_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
                && 1 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("[DT Core | INFO | {}] {}", $crate::util::datetime::get_fmt_datetime(), format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_warning {
        ($($arg:tt)*) => {
            if (2 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                println!("[DT Core | WARNING | {}] {}", $crate::util::datetime::get_fmt_datetime(), format!($($arg)*));
            }
        };
    }

    #[macro_export]
    macro_rules! log_error {
        ($($arg:tt)*) => {
            if (3 >= $crate::util::logger::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
                eprintln!("[DT Core | ERROR | {}] {}", $crate::util::datetime::get_fmt_datetime(), format!($($arg)*));
            }
        };
    }

    pub use log_info;
    pub use log_warning;
    pub use log_error;
}