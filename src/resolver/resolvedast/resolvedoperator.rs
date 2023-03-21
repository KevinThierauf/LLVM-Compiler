use crate::module::Operator;
use crate::resolver::resolvedast::resolvedexpr::{ResolvedExpr, ResolvedExprType};
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct ResolvedOperator {
    pub operator: Operator,
    pub operands: Box<[ResolvedExpr]>,
    pub expressionType: Type,
}

impl StatementType for ResolvedOperator {
}

impl ResolvedExprType for ResolvedOperator {
    fn getExpressionType(&self) -> Type {
        return self.expressionType.to_owned();
    }
}
