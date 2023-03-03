use std::cmp::{max, min, Ordering};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

use crate::module::{FilePos, FileRange, Module, SourceFile, Token, TokenType};

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
        return write!(f, "Module({:?}):{}", self.module.moduleId, self.getTokenIndex());
    }
}

thread_local! {
    static END_TOKEN: Rc<Token> = Rc::new(Token::new(TokenType::SemiColan, FileRange::new(FilePos::new(SourceFile::fromSource(PathBuf::new(), String::new()), 0), 0)));
}

impl ModulePos {
    pub fn getModule(&self) -> &Rc<Module> {
        return &self.module;
    }

    pub fn getTokenIndex(&self) -> usize {
        return self.tokenIndex;
    }

    pub fn setTokenIndex(&mut self, index: usize) {
        debug_assert!(index <= self.module.tokenVec.len());
        self.tokenIndex = index;
    }

    pub fn getToken(&self) -> &Token {
        return if self.tokenIndex == self.getModule().getTokenVector().len() {
            // END_TOKEN is only valid for the lifetime of the current thread
            // since Token is not sync, &Token cannot be sent to another thread so it's lifetime is effectively 'static
            END_TOKEN.with(|reference| unsafe { std::mem::transmute(reference.deref()) })
        } else {
            &self.module.getTokenVector()[self.tokenIndex]
        };
    }

    pub fn getRangeWithLength(&self, length: usize) -> ModuleRange {
        return self.module.getModuleRange(self.getTokenIndex()..self.getTokenIndex() + length);
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleRange {
    pub(in super) startPos: ModulePos,
    pub(in super) length: usize,
}

impl Debug for ModuleRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Module({:?}):{} (through {})", self.startPos.getModule().moduleId, self.getStartIndex(), self.getEndIndex());
        // return write!(f, "Module({:?}):{} (through {}) \"{}\"", self.startPos.getModule().moduleId, self.getStartIndex(), self.getEndIndex(), self.getSource());
    }
}

impl ModuleRange {
    pub fn getModule(&self) -> &Rc<Module> {
        return self.startPos.getModule();
    }

    pub fn getSource(&self) -> String {
        return self.getTokens().iter().map(|v| v.getSourceRange().getSourceInRange()).fold(String::new(), |a, b| a + b);
    }

    pub fn getStartPos(&self) -> ModulePos {
        return self.startPos.to_owned();
    }

    pub fn getEndPos(&self) -> ModulePos {
        return self.getModule().getModulePos(self.getEndIndex());
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

    pub fn getTokens(&self) -> &[Token] {
        return &self.getModule().getTokenVector()[self.getStartIndex()..self.getEndIndex()];
    }

    pub fn setStartIndex(&mut self, index: usize) {
        debug_assert!(index <= self.getEndIndex());
        self.startPos.tokenIndex = index;
    }

    pub fn setEndIndex(&mut self, index: usize) {
        debug_assert!(index >= self.getStartIndex());
        self.length = index - self.getStartIndex();
    }

    #[must_use]
    pub fn getCombined(&self, range: &ModuleRange) -> ModuleRange {
        assert!(Rc::ptr_eq(self.getModule(), range.getModule()), "cannot combine range across modules");
        let module = range.getModule().to_owned();
        let startIndex = min(self.getStartIndex(), range.getStartIndex());
        let endIndex = max(self.getEndIndex(), range.getEndIndex());
        return ModuleRange {
            startPos: module.getModulePos(startIndex),
            length: endIndex - startIndex,
        }
    }
}
