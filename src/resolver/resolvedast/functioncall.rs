use crate::resolver::function::Function;
use crate::resolver::resolvedast::resolvedexpr::{ResolvedExpr, ResolvedExprType};
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Function,
    pub argVec: Vec<ResolvedExpr>,
}

impl StatementType for FunctionCall {}

impl ResolvedExprType for FunctionCall {
    fn getExpressionType(&self) -> Type {
        return self.function.returnType.to_owned();
    }
}
