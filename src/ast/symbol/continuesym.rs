use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::SymbolType;

#[derive(Debug)]
pub struct ContinueSym {
    pub range: ModuleRange,
    pub label: Option<Label>
}

impl SymbolType for ContinueSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
