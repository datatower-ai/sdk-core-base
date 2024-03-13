use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) fn get_time_since_epoch() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward")
}

pub(crate) fn get_hour_since_epoch() -> u64 {
    let since_epoch_sec = get_time_since_epoch().as_secs();
    since_epoch_sec - since_epoch_sec % 3600
}