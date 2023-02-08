use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::SymbolType;

pub struct BreakSym {
    range: ModuleRange,
    label: Option<Label>
}

impl SymbolType for BreakSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
