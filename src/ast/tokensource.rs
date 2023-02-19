use std::cmp::min;
use std::rc::Rc;

use crate::ast::symbol::Symbol;
use crate::ast::tokensource::tokenparser::parseTokenVec;
use crate::module::{Module, TokenType, TokenTypeDiscriminants};
use crate::module::modulepos::ModulePos;

pub mod conflictresolution;
pub mod tokenparser;
pub mod symbolparser;
mod matchers;

pub enum ASTError {
    // misc match fail
    MatchFailed(ModulePos),
    // failed to match symbol
    ExpectedSymbol(ModulePos),
    // position, expected exact token
    ExpectedToken(ModulePos, TokenType),
    // position, expected token type discriminant
    ExpectedTokenDiscriminant(ModulePos, TokenTypeDiscriminants),
    // expected token to be exclusive in module, found extraneous symbols
    ExpectedExclusive(ModulePos, Option<TokenTypeDiscriminants>),
    // conflict resolution returned multiple
    MultipleConflict(ModulePos, Vec<String>),
    // conflict resolution returned none
    EliminatedConflict(ModulePos, Vec<String>),
    // all potential matches failed
    MatchOptionsFailed(ModulePos, Vec<ASTError>),
}

impl ASTError {
    pub fn getErrorMessage(&self) -> String {
        return match self {
            ASTError::MatchFailed(pos) => format!("failed to find match at {pos:?}"),
            ASTError::ExpectedSymbol(pos) => format!("failed to match symbol (at {pos:?})"),
            ASTError::ExpectedToken(pos, expected) => format!("expected {expected:?} at {pos:?}, found {:?}", pos.getToken()),
            ASTError::ExpectedTokenDiscriminant(pos, expected) => format!("expected {expected:?} at {pos:?}, found {:?}", pos.getToken()),
            ASTError::ExpectedExclusive(pos, expected) => if let Some(expected) = expected {
                format!("expected single {expected:?} token, found extra {:?} token", pos.getToken())
            } else {
                format!("expected single token, found extra {:?} token", pos.getToken())
            },
            ASTError::MultipleConflict(pos, options) => format!("conflict resolution returned multiple potential symbols at {pos:?}: {options:?}"),
            ASTError::EliminatedConflict(pos, options) => format!("cannot determine appropriate symbol from multiple conflicting matches at {pos:?}; all possibilities eliminated ({options:?})"),
            ASTError::MatchOptionsFailed(pos, options) => format!("all potential matches failed at {pos:?}{}", options.iter().map(|err| format!("\n\t{}", err.getDisplayMessage().replace('\n', "\n\t"))).collect::<Vec<String>>().join("")),
        };
    }

    pub fn getModulePos(&self) -> &ModulePos {
        return match self {
            ASTError::MatchFailed(pos) |
            ASTError::ExpectedSymbol(pos) |
            ASTError::ExpectedToken(pos, _) |
            ASTError::ExpectedTokenDiscriminant(pos, _) |
            ASTError::ExpectedExclusive(pos, _) |
            ASTError::MultipleConflict(pos, _) |
            ASTError::EliminatedConflict(pos, _) => pos,
            ASTError::MatchOptionsFailed(pos, _) => pos,
        };
    }

    pub fn getTokenSource(&self) -> (String, usize) {
        const PREVIOUS_TOKENS: usize = 5;
        const NEXT_TOKENS: usize = 5;

        let pos = self.getModulePos();
        let mut range = pos.getRange(min(pos.getModule().getTokenVector().len(), pos.getTokenIndex() + NEXT_TOKENS) - pos.getTokenIndex());
        range.setStartIndex(range.getStartIndex().saturating_sub(PREVIOUS_TOKENS));

        let mut source = String::new();
        let mut sourceIndex = 0;
        let tokens = range.getTokens();
        for tokenIndex in 0..tokens.len() {
            let token = &tokens[tokenIndex];
            source += token.getSourceRange().getSourceInRange();
            if tokenIndex == pos.getTokenIndex() - range.getStartIndex() {
                sourceIndex = source.len();
            }
        }

        return (source.replace('\n', " ").replace('\r', ""), sourceIndex);
    }

    pub fn getDisplayMessage(&self) -> String {
        let (source, index) = self.getTokenSource();
        return format!("error: {}\n\t(at {:?})\n\t> \"{}\"\n\t {}", self.getErrorMessage(), self.getModulePos(), source, " ".repeat(index) + "^");
    }
}

pub fn parseModule(module: Rc<Module>) -> Result<Vec<Symbol>, ASTError> {
    return parseTokenVec(module);
}
