pub mod message;

use std::sync::Arc;
use parking_lot::Mutex;
use crate::module::logger::message::MessageType;
use crate::module::Module;

struct LogInternal {
    returnedMessages: Vec<Box<dyn MessageType>>,
    missingGroups: usize,
}

impl LogInternal {
    fn returnGroup(&mut self, messageVec: &mut Vec<Box<dyn MessageType>>) {
        self.missingGroups -= 1;
        self.returnedMessages.append(messageVec);
    }

    fn requestGroup(&mut self, logger: Logger) -> LogGroup {
        self.missingGroups += 1;
        return LogGroup {
            logger,
            messageVec: Vec::new(),
        }
    }

    fn getMessages(self) -> Vec<Box<dyn MessageType>> {
        assert_eq!(self.missingGroups, 0);
        return self.returnedMessages;
    }
}

#[derive(Clone)]
pub struct Logger {
    internal: Arc<Mutex<LogInternal>>,
}

impl Logger {
    pub fn getNextLogGroup(&mut self) -> LogGroup {
        return self.internal.lock().requestGroup(self.to_owned());
    }
}

pub struct LogGroup {
    logger: Logger,
    messageVec: Vec<Box<dyn MessageType>>
}

impl LogGroup {
    pub fn log(&mut self, message: impl MessageType) {
        if message.hasSilenced() {
            return;
        }
        self.messageVec.push(Box::new(message));
    }
}

impl Drop for LogGroup {
    fn drop(&mut self) {
        self.logger.internal.lock().returnGroup(&mut self.messageVec);
    }
}
