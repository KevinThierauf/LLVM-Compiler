use std::rc::Rc;
use crate::ast::ASTError;
use crate::ast::symbol::Symbol;
use crate::module::{Module, Token};

pub fn parseTokenVec(module: Rc<Module>, tokens: &Vec<Token>) -> Result<Vec<Symbol>, ASTError> {
    todo!()
}
