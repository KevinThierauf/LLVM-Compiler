pub mod conflictresolution;
pub mod tokenparser;
pub mod symbolparser;
mod matchers;

use std::rc::Rc;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::tokenparser::parseTokenVec;
use crate::module::Module;

pub enum ASTError {
    NoMatch,
}

impl ASTError {
    pub fn getDisplayMessage(&self) -> String {
        return match self {
            ASTError::NoMatch => "failed to match symbol".to_owned(),
        };
    }
}

pub fn parseModule(module: Rc<Module>) -> Result<Vec<Symbol>, ASTError> {
    return parseTokenVec(module.to_owned(), module.getTokenVector());
}
