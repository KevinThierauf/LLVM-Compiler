use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ReadExpr {
    pub range: ModuleRange,
}

impl SymbolType for ReadExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for ReadExpr {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}
