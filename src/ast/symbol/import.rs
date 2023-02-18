use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::ast::symbol::SymbolType;

#[derive(Debug)]
pub struct ImportSym {
    pub range: ModuleRange,
    pub packageName: ModulePos,
    pub localName: Option<ModulePos>,
}

impl SymbolType for ImportSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
