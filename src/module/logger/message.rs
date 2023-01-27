use crate::source::filepos::FilePos;

pub enum MessageLevel {
    Fatal,
    Error,
    Warning,
    MinorWarning,
    Info,
    Success,
}

pub trait MessageType: 'static {
    fn hasSilenced(&self) -> bool;
    fn getLocation(&self) -> Option<FilePos>;
    fn getLevel(&self) -> MessageLevel;
}

pub struct CommonMessage<T: MessageType> {
    level: MessageLevel,
    messageType: T,
    location: Option<FilePos>,
}

pub enum CommonMessageType {
}
