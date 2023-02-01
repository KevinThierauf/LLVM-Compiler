use crate::module::symbol::expr::Expr;
use label::Label;

pub mod loopexpr;
pub mod whileexpr;
pub mod forexpr;
pub mod label;

pub trait LoopType {
    fn getConditional(&self) -> Expr;
    fn getLabel(&self) -> Option<&Label>;
}
