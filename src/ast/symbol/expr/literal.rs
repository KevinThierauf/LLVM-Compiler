use crate::ast::symbol::expr::ExprType;

pub mod literalbool;
pub mod literalarray;
pub mod literaltuple;
pub mod literalfloat;
pub mod literalstring;
pub mod literalinteger;
pub mod literalchar;
pub mod literalvoid;

pub trait LiteralType: ExprType {
}
