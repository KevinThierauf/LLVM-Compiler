use crate::module::symbol::expr::Expr;
use crate::module::symbol::expr::label::Label;

pub mod loopexpr;
pub mod whileexpr;

pub trait LoopType {
    fn getConditional(&self) -> Expr;
    fn getLabel(&self) -> Option<&Label>;
}
