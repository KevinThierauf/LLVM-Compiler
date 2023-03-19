use crate::module::Operator;
use crate::resolver::resolvedast::resolvedexpr::{ResolvedExpr, ResolvedExprType};
use crate::resolver::resolvedast::statement::{Statement, StatementType};
use crate::resolver::typeinfo::Type;

pub struct ResolvedOperator {
    pub operator: Operator,
    pub operands: Box<[ResolvedExpr]>,
}

impl StatementType for ResolvedOperator {
}

impl ResolvedExprType for ResolvedOperator {
    fn getExpressionType(&self) -> Type {
        return self.operands[0].getExpressionType();
    }
}
