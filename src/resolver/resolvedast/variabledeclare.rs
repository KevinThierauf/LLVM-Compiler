use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct VariableDeclare {
    ty: Type
}

impl StatementType for VariableDeclare {}

impl ResolvedExprType for VariableDeclare {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }

    fn isAssignable(&self) -> bool {
        return true;
    }
}
