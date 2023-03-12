use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::ExprType;
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
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::Variable(*self);
    }
}
