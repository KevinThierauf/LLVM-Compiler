use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct DefaultPointer {
    pub ty: Type,
}

impl StatementType for DefaultPointer {}

impl ResolvedExprType for DefaultPointer {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }
}
