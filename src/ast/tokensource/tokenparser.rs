use std::rc::Rc;

use crate::ast::ASTError;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::matchers::getMatchSymbolsAll;
use crate::ast::tokensource::symbolparser::MatchType;
use crate::module::Module;

pub fn parseTokenVec(module: Rc<Module>) -> Result<Vec<Symbol>, ASTError> {
    return getMatchSymbolsAll().getMatch(module.getModulePos(0)).map(|matchValue| matchValue.take().1).ok_or(ASTError::NoMatch);
}
