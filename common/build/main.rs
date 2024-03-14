cfg_if::cfg_if! {
    if #[cfg(not(any(feature = "log-consumer", feature = "db-cache-consumer")))] {
        fn main() {
            compile_error!("At least one consumer needs to be included by --feature!");
        }
    } else {
        fn main() {}
    }
}