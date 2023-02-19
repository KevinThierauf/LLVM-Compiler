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
        Some(Self {
            range: match components.get(0)? {
                OperationComponent::Operator(r, _) => r.to_owned(),
                OperationComponent::Expression(v) => v.getRange().to_owned()
            },
            operands: Box::new([]),
            operator: Operator::Increment,
        })
    }
}

impl SymbolType for OperatorExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for OperatorExpr {}

#[derive(Debug)]
pub enum OperationComponent {
    Operator(ModuleRange, Operator),
    Expression(Expr),
}
