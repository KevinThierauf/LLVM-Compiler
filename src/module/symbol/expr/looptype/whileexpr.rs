use crate::module::symbol::expr::Expr;
use crate::module::symbol::expr::label::Label;
use crate::module::symbol::expr::looptype::LoopType;

pub struct WhileExpr {
    label: Option<Label>
}

impl LoopType for WhileExpr {
    fn getConditional(&self) -> Expr {
        todo!()
    }

    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}