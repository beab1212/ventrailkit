//! Status codes returned by wire decoders.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Truncated,
    InvalidArgument,
    OutOfRange,
    ChecksumMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusMessage {
    pub code: Status,
    pub detail: &'static str,
}

impl StatusMessage {
    pub fn ok() -> Self { Self { code: Status::Ok, detail: "ok" } }
    pub fn fail(code: Status, detail: &'static str) -> Self { Self { code, detail } }
    pub fn is_ok(&self) -> bool { self.code == Status::Ok }
}

impl From<Result<(), StatusMessage>> for StatusMessage {
    fn from(value: Result<(), StatusMessage>) -> Self {
        match value {
            Ok(()) => StatusMessage::ok(),
            Err(e) => e,
        }
    }
}

pub type ResultStatus = Result<(), StatusMessage>;
