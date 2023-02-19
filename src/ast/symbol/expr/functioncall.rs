use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::{ModulePos, ModuleRange};

#[derive(Debug)]
pub struct FunctionCallExpr {
    pub range: ModuleRange,
    pub functionName: ModulePos,
    pub argVec: Vec<Expr>,
}

impl SymbolType for FunctionCallExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for FunctionCallExpr {}