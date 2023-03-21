use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::{ModulePos, ModuleRange};

#[derive(Debug)]
pub struct ConstructorCallExpr {
    pub range: ModuleRange,
    pub typeName: ModulePos,
    pub argVec: Vec<Expr>,
}

impl SymbolType for ConstructorCallExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for ConstructorCallExpr {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}