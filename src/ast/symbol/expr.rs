use downcast_rs::{Downcast, impl_downcast};
use crate::ast::symbol::{Symbol, SymbolType};

pub mod operatorexpr;
pub mod functioncall;
pub mod variableexpr;
pub mod literal;
pub mod variabledeclaration;

pub trait ExprType: 'static + SymbolType + Downcast {
    fn toSymbol(self: Box<Self>) -> Symbol;
}

impl_downcast!(ExprType);

pub type Expr = Box<dyn ExprType>;
