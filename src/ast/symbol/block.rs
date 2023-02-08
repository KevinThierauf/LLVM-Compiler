use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::{Symbol, SymbolType};

pub struct BlockSym {
    range: ModuleRange,
    symbolVec: Vec<Symbol>,
}

impl SymbolType for BlockSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
