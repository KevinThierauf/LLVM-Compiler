use crate::ast::symbol::SymbolType;
use crate::module::modulepos::{ModulePos, ModuleRange};

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
