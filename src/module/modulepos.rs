use std::rc::Rc;
use crate::module::Module;

#[derive(Clone)]
pub struct ModulePos {
    pub(in super) module: Rc<Module>,
    pub(in super) tokenIndex: usize,
}

impl ModulePos {
    pub fn getModule(&self) -> &Rc<Module> {
        return &self.module;
    }

    pub fn getTokenIndex(&self) -> usize {
        return self.tokenIndex;
    }
}

#[derive(Clone)]
pub struct ModuleRange {
    pub(in super) startPos: ModulePos,
    pub(in super) length: usize,
}

impl ModuleRange {
    pub fn getModule(&self) -> &Rc<Module> {
        return self.startPos.getModule();
    }

    pub fn getStartIndex(&self) -> usize {
        return self.startPos.getTokenIndex();
    }

    pub fn getEndIndex(&self) -> usize {
        return self.getStartIndex() + self.getLength();
    }

    pub fn getLength(&self) -> usize {
        return self.length;
    }
}
