use std::ops::Range;
use std::rc::Rc;
use crate::module::modulepos::{ModulePos, ModuleRange};

use crate::module::tokenparser::TokenParser;
use crate::source::filepos::SourceFile;
use crate::source::token::Token;

mod tokenparser;
pub mod symbol;
pub mod modulepos;
pub mod logger;
pub mod typeinfo;

pub struct Module {
    tokenVec: Vec<Token>,
}

impl Module {
    pub fn new(tokenVec: Vec<Token>) -> Rc<Self> {
        let mut module = Self {
            tokenVec,
        };
        TokenParser::new(&mut module).parse();
        return Rc::new(module);
    }

    pub fn parse(sourceFile: SourceFile) -> Rc<Self> {
        // return Self::new(parseSource(sourceFile));
        todo!()
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