pub mod conflictresolution;
pub mod tokenparser;
pub mod symbolresolver;

use std::rc::Rc;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::tokenparser::TokenParser;
use crate::module::Module;

pub enum ASTError {
    NoMatch,
}

pub fn parseModule(module: Rc<Module>) -> Result<Vec<Symbol>, ASTError> {
    return TokenParser::new(module.getTokenVector()).parse();
}
