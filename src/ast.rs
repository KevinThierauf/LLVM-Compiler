use std::rc::Rc;

use crate::ast::symbol::Symbol;
pub use crate::ast::tokensource::ASTError;
use crate::ast::tokensource::parseModule;
use crate::module::Module;

mod tokensource;
pub mod symbol;
pub mod typeinfo;

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
}
