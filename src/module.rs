use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

use crate::module::modulepos::{ModulePos, ModuleRange};
pub use crate::module::source::filepos::*;
pub use crate::module::source::ParseError;
use crate::module::source::parseSource;
pub use crate::module::source::token::*;

pub mod modulepos;
mod source;

static MODULE_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Module {
    moduleId: usize,
    tokenVec: Vec<Token>,
}

impl Module {
    pub fn newFrom(mut tokenVec: Vec<Token>) -> Rc<Self> {
        tokenVec.retain(|token| !matches!(token.getTokenType(), TokenType::Comment(_)));
        return Rc::new(Self {
            moduleId: MODULE_INDEX.fetch_add(1, Relaxed),
            tokenVec,
        });
    }

    pub fn new(sourceFile: SourceFile) -> Result<Rc<Self>, ParseError> {
        return Ok(Self::newFrom(parseSource(sourceFile)?));
    }

    pub fn getTokenVector(&self) -> &Vec<Token> {
        return &self.tokenVec;
    }

    pub fn getModulePos(self: &Rc<Self>, tokenIndex: usize) -> ModulePos {
        debug_assert!(tokenIndex <= self.tokenVec.len());
        return ModulePos {
            module: self.to_owned(),
            tokenIndex,
        };
    }

    pub fn getModuleRange(self: &Rc<Self>, range: Range<usize>) -> ModuleRange {
        let startPos = self.getModulePos(range.start);
        debug_assert!(range.end <= self.tokenVec.len());
        debug_assert!(range.start <= range.end);
        return ModuleRange {
            startPos,
            length: range.end - range.start,
        };
    }
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.moduleId);
    }
}
