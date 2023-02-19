use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ParenthesisExpr {
    pub range: ModuleRange,
    pub expression: Expr,
}

impl ExprType for ParenthesisExpr {}

impl SymbolType for ParenthesisExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
