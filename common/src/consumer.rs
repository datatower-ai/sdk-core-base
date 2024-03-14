
#[cfg(feature = "log-consumer")]
pub mod log;

#[cfg(feature = "db-cache-consumer")]
pub mod database_cache;

pub(crate) const MEM_KEY: &'static str = "consumer";

pub trait Consumer {
    fn add(self: &mut Self, event: serde_json::Map<String, serde_json::Value>) -> bool;

    fn flush(self: &mut Self);

    fn close(self: &mut Self);
}