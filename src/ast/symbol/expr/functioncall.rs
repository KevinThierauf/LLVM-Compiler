use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;

pub struct FunctionCallExpr {
    range: ModuleRange,
    functionName: ModuleRange,
    argVec: Vec<Expr>,
}

impl SymbolType for FunctionCallExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for FunctionCallExpr {
}