use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;

pub struct Loop {
    label: Option<Label>,
}

impl LoopType for Loop {
    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}
