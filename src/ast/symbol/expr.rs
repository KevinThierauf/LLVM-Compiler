use crate::ast::symbol::{Symbol, SymbolType};

pub mod operatorexpr;
pub mod functioncall;
pub mod variableexpr;
pub mod literal;
pub mod variabledeclaration;

pub trait ExprType: 'static + SymbolType {
    fn toSymbol(self: Box<Self>) -> Symbol;
}

pub type Expr = Box<dyn ExprType>;
