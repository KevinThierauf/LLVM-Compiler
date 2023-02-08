use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::SymbolType;

pub struct ImportSym {
    range: ModuleRange,
    packageName: ModuleRange,
    localName: Option<ModuleRange>,
}

impl SymbolType for ImportSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
