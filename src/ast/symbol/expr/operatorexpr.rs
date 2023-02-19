use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;
use crate::module::Operator;

#[derive(Debug)]
pub struct OperatorExpr {
    pub range: ModuleRange,
    pub operands: Box<[Expr]>,
    pub operator: Operator,
}

impl OperatorExpr {
    pub fn getFromComponents(components: Vec<OperationComponent>) -> Option<Self> {
        todo!()
    }
}

impl SymbolType for OperatorExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for OperatorExpr {
}

pub enum OperationComponent {
    Operator(Operator),
    Expression(Expr),
}
