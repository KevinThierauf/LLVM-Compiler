use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ReturnSym {
    pub range: ModuleRange,
    pub value: Option<Expr>,
}

impl SymbolType for ReturnSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
