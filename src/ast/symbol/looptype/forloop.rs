use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ForLoop {
    range: ModuleRange,
    label: Option<Label>,
}

impl SymbolType for ForLoop {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LoopType for ForLoop {
    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}
