use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct VariableExpr {
    pub range: ModuleRange,
}

impl SymbolType for VariableExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for VariableExpr {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}
