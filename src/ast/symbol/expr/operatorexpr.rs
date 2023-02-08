use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;
use crate::module::Operator;

pub struct OperatorExpr {
    range: ModuleRange,
    operands: Box<[Expr]>,
    operator: Operator,
}

impl SymbolType for OperatorExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for OperatorExpr {
}
