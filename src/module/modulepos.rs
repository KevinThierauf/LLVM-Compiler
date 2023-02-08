use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::module::Module;

#[derive(Clone, Hash)]
pub struct ModulePos {
    pub(in super) module: Rc<Module>,
    pub(in super) tokenIndex: usize,
}

impl PartialEq<Self> for ModulePos {
    fn eq(&self, other: &Self) -> bool {
        return Rc::ptr_eq(&self.module, &other.module) && self.tokenIndex == other.tokenIndex;
    }
}

impl Eq for ModulePos {}

impl PartialOrd<Self> for ModulePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for ModulePos {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = Rc::as_ptr(&self.module).cmp(&Rc::as_ptr(&other.module));
        if let Ordering::Equal = cmp {
            self.tokenIndex.cmp(&other.tokenIndex)
        } else {
            cmp
        }
    }
}

impl Debug for ModulePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Module{:?}:{}", self.module.moduleId, self.getTokenIndex());
    }
}

impl ModulePos {
    pub fn getModule(&self) -> &Rc<Module> {
        return &self.module;
    }

    pub fn getTokenIndex(&self) -> usize {
        return self.tokenIndex;
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleRange {
    pub(in super) startPos: ModulePos,
    pub(in super) length: usize,
}

impl Debug for ModuleRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Module{:?}:{} (through {})", self.startPos.getModule().moduleId, self.getStartIndex(), self.getEndIndex());
    }
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
