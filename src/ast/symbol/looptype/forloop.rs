use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;

pub struct ForLoop {
    label: Option<Label>,
}

impl LoopType for ForLoop {
    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}
