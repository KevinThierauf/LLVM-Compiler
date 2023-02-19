use crate::ast::symbol::SymbolType;

pub mod operatorexpr;
pub mod functioncall;
pub mod variableexpr;
pub mod literal;
pub mod parenthesisexpr;
pub mod variabledeclaration;
pub mod memberaccessexpr;
pub mod methodcallexpr;

pub trait ExprType: SymbolType {}

pub type Expr = Box<dyn ExprType>;
