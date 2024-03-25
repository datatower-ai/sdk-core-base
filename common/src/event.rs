use crate::util::error::Result;

mod data_verification;
pub(crate) mod processing;

pub type Event = serde_json::Map<String, serde_json::Value>;
pub type BoxedEvent = Box<Event>;


pub(crate) fn init() -> Result<()> {
    data_verification::init()?;
    Ok(())
}
