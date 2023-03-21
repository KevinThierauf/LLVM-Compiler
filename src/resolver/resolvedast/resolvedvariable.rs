use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct ResolvedVariable {
    pub variableName: String,
    pub ty: Type
}

impl StatementType for ResolvedVariable {
}

impl ResolvedExprType for ResolvedVariable {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }

    fn isAssignable(&self) -> bool {
        return true;
    }
}