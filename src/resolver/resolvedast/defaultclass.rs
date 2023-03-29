use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct DefaultClass {
    pub ty: Type,
}

impl StatementType for DefaultClass {}

impl ResolvedExprType for DefaultClass {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }
}