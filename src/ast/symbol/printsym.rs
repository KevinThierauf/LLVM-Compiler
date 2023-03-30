use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct PrintSym {
    pub range: ModuleRange,
    pub expr: Expr,
}

impl SymbolType for PrintSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
