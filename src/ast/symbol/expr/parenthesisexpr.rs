use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;

pub struct ParenthesisExpr {
    range: ModuleRange,
    expression: Expr
}

impl ExprType for ParenthesisExpr {
}

impl SymbolType for ParenthesisExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
