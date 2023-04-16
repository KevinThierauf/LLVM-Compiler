use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::Type;

#[derive(Debug, Default)]
pub struct ReadExpr {}

impl StatementType for ReadExpr {}

impl ResolvedExprType for ReadExpr {
    fn getExpressionType(&self) -> Type {
        return INTEGER_TYPE.to_owned();
    }
}
