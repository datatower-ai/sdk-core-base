mod log;

pub(crate) const MEM_KEY: &str = "consumer";

trait Consumer {
    fn add(mut self: &mut Self, event: serde_json::Map<String, serde_json::Value>) -> bool;

    fn flush(mut self: &mut Self);

    fn close(self: Self);
}