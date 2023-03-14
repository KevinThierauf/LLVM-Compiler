use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::ast::symbol::Symbol;
pub use crate::ast::tokensource::ASTError;
use crate::ast::tokensource::parseModule;
use crate::module::Module;

mod tokensource;
pub mod symbol;
pub mod visibility;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    symbolVec: Vec<Symbol>,
}

impl AbstractSyntaxTree {
    pub fn newFrom(symbolVec: Vec<Symbol>) -> Rc<Self> {
        return Rc::new(Self {
            symbolVec,
        });
    }

    pub fn new(module: Rc<Module>) -> Result<Rc<Self>, ASTError> {
        return Ok(Self::newFrom(parseModule(module)?));
    }

    pub fn getSymbols(&self) -> &Vec<Symbol> {
        return &self.symbolVec;
    }

    pub fn getPos(self: &Rc<Self>, index: usize) -> SymbolPos {
        return SymbolPos::new(self.to_owned(), index);
    }
}

#[derive(Clone)]
pub struct SymbolPos {
    ast: Rc<AbstractSyntaxTree>,
    index: usize,
}

impl PartialEq<Self> for SymbolPos {
    fn eq(&self, other: &Self) -> bool {
        return Rc::ptr_eq(&self.ast, &other.ast) && self.index == other.index;
    }
}

impl Eq for SymbolPos {}

impl PartialOrd<Self> for SymbolPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for SymbolPos {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = Rc::as_ptr(&self.ast).cmp(&Rc::as_ptr(&other.ast));
        if let Ordering::Equal = cmp {
            self.index.cmp(&other.index)
        } else {
            cmp
        }
    }
}

impl Debug for SymbolPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "\"{}\"", self.getSymbol().getSymbolType().getRange().getSource());
    }
}

impl SymbolPos {
    fn new(ast: Rc<AbstractSyntaxTree>, index: usize) -> Self {
        debug_assert!(index < ast.symbolVec.len());
        return Self {
            ast,
            index,
        };
    }

    pub fn getSymbol(&self) -> &Symbol {
        return &self.ast.symbolVec[self.index];
    }
}
