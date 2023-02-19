use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct MethodCallExpr {
    pub range: ModuleRange,
    pub structureName: Expr,
    pub functionCall: FunctionCallExpr,
}

impl SymbolType for MethodCallExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for MethodCallExpr {}