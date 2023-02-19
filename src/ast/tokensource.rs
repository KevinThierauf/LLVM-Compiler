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
    // conflict resolution returned multiple
    MultipleConflict,
    // conflict resolution returned none
    EliminatedConflict,
}

impl ASTError {
    pub fn getDisplayMessage(&self) -> String {
        return match self {
            ASTError::NoMatch => "failed to match symbol".to_owned(),
            ASTError::MultipleConflict => "conflict resolution returned multiple potential symbols".to_owned(),
            ASTError::EliminatedConflict => "could not determine appropriate symbol from multiple conflicting matches".to_owned(),
        };
    }
}

pub fn parseModule(module: Rc<Module>) -> Result<Vec<Symbol>, ASTError> {
    return parseTokenVec(module);
}
