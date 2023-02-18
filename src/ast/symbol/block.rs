use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::{Symbol, SymbolType};

#[derive(Debug)]
pub struct BlockSym {
    pub range: ModuleRange,
    pub symbolVec: Vec<Symbol>,
}

impl SymbolType for BlockSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
