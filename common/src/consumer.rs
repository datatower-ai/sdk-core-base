
#[cfg(feature = "log-consumer-server")]
pub mod log;

#[cfg(feature = "db-cache-consumer-client")]
pub mod database_cache;

#[cfg(feature = "async-upload-server")]
pub mod async_upload;

pub(crate) const MEM_KEY: &'static str = "consumer";

pub trait Consumer {
    fn add(self: &mut Self, event: serde_json::Map<String, serde_json::Value>) -> bool;

    fn flush(self: &mut Self);

    fn close(self: &mut Self);
}