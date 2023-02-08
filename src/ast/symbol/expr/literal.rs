pub mod literalbool;
pub mod literalarray;
pub mod literaltuple;
pub mod literalfixed;
pub mod literalstring;
pub mod literalinteger;
pub mod literalchar;
pub mod literalvoid;

use crate::ast::symbol::expr::ExprType;
use crate::ast::typeinfo::Type;

pub trait LiteralType: ExprType {
    fn getLiteralType(&self) -> Type;
}
