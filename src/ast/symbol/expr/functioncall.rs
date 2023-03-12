use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::{Expr, ExprType};
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

impl ExprType for FunctionCallExpr {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::FunctionCall(*self);
    }
}