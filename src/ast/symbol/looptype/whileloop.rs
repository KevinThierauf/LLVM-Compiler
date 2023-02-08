use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;

pub struct WhileLoop {
    label: Option<Label>
}

impl LoopType for WhileLoop {
    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}
