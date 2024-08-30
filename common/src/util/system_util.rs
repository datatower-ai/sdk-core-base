#[cfg(windows)]
pub(crate) const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
pub(crate) const LINE_ENDING: &'static str = "\n";

pub(crate) const LINE_ENDING_LENGTH: usize = LINE_ENDING.as_bytes().len();