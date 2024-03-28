use crate::util::error::Result;

mod data_verification;
pub(crate) mod processing;
pub(crate) mod common_properties;

pub type Event = serde_json::Map<String, serde_json::Value>;
pub type BoxedEvent = Box<Event>;


pub(crate) fn init() -> Result<()> {
    data_verification::init()?;
    Ok(())
}
