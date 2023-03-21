use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct ConstructorCall {
    pub ty: Type,
}

impl StatementType for ConstructorCall {}

impl ResolvedExprType for ConstructorCall {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }
}
