use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct DefaultValue {
    pub ty: Type,
}

impl StatementType for DefaultValue {}

impl ResolvedExprType for DefaultValue {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }
}
