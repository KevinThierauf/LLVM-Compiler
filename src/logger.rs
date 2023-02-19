use std::mem::swap;
use std::sync::Arc;

use hashbrown::HashSet;
use parking_lot::Mutex;

use crate::logger::message::{CoreMessageType, MessageType};

pub mod message;

struct LogInternal {
    returnedMessages: Vec<Box<dyn MessageType>>,
    missingGroups: usize,
    silencedCoreMessages: HashSet<CoreMessageType>,
}

#[derive(Clone)]
pub struct Logger {
    internal: Arc<Mutex<LogInternal>>,
}

impl Logger {
    pub fn getNextLogGroup(&mut self) -> LogGroup {
        let mut internal = self.internal.lock();
        internal.missingGroups += 1;
        return LogGroup {
            logger: self.to_owned(),
            messageVec: Vec::new(),
        };
    }

    fn returnGroup(&mut self, messageVec: &mut Vec<Box<dyn MessageType>>) {
        let mut internal = self.internal.lock();
        internal.missingGroups -= 1;
        internal.returnedMessages.append(messageVec);
    }

    pub fn isSilenced(&self, coreMessage: CoreMessageType) -> bool {
        return self.internal.lock().silencedCoreMessages.contains(&coreMessage);
    }

    pub fn setSilenced(&mut self, coreMessage: CoreMessageType, silenced: bool) {
        let messages = &mut self.internal.lock().silencedCoreMessages;
        if silenced {
            messages.insert(coreMessage);
        } else {
            messages.remove(&coreMessage);
        }
    }

    pub fn getMessages(self) -> Vec<Box<dyn MessageType>> {
        let mut vec = {
            let mut internal = self.internal.lock();
            assert_eq!(internal.missingGroups, 0);
            let mut vec = Vec::new();
            swap(&mut vec, &mut internal.returnedMessages);
            vec
        };
        vec.retain(|message| !message.hasSilenced(&self));
        return vec;
    }
}

pub struct LogGroup {
    logger: Logger,
    messageVec: Vec<Box<dyn MessageType>>,
}

impl LogGroup {
    pub fn log(&mut self, message: impl MessageType) {
        self.messageVec.push(Box::new(message));
    }
}

impl Drop for LogGroup {
    fn drop(&mut self) {
        self.logger.returnGroup(&mut self.messageVec);
    }
}
