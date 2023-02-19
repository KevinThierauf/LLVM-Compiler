use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct MemberAccessExpr {
    pub range: ModuleRange,
    pub structureName: Expr,
    pub variable: VariableExpr,
}

impl SymbolType for MemberAccessExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for MemberAccessExpr {}