cfg_if::cfg_if! {
    if #[cfg(not(any(feature = "log-consumer")))] {
        fn main() {
            compile_error!("At least one consumer needs to include as feature!");
        }
    } else {
        fn main() {}
    }
}