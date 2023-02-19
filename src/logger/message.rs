use crate::logger::Logger;
use crate::module::modulepos::ModulePos;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MessageLevel {
    Fatal,
    Error,
    Warning,
    MinorWarning,
    Info,
    Success,
}

pub trait MessageType: 'static {
    fn hasSilenced(&self, logger: &Logger) -> bool;
    fn getLocation(&self) -> Option<ModulePos>;
    fn getLevel(&self) -> MessageLevel;
}

pub struct CoreMessage {
    level: MessageLevel,
    location: Option<ModulePos>,
    messageType: CoreMessageType,
}

impl CoreMessage {
    pub fn new(level: MessageLevel, location: Option<ModulePos>, messageType: CoreMessageType) -> Self {
        return Self {
            level,
            location,
            messageType,
        };
    }

    pub fn getMessageType(&self) -> &CoreMessageType {
        return &self.messageType;
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CoreMessageType {}

impl CoreMessageType {
    pub fn getParentMessageType(&self) -> &'static [CoreMessageType] {
        return match self {
            _ => &[],
        };
    }

    pub fn hasSilenced(&self, logger: &Logger) -> bool {
        return logger.isSilenced(*self);
    }
}

impl MessageType for CoreMessage {
    fn hasSilenced(&self, logger: &Logger) -> bool {
        return self.messageType.hasSilenced(logger);
    }

    fn getLocation(&self) -> Option<ModulePos> {
        return self.location.to_owned();
    }

    fn getLevel(&self) -> MessageLevel {
        return self.level;
    }
}
