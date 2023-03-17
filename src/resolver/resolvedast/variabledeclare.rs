use crate::resolver::resolvedast::resolvedexpr::ResolvedExprType;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::Type;

pub struct VariableDeclare {
    ty: Type
}

impl StatementType for VariableDeclare {}

impl ResolvedExprType for VariableDeclare {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }
}
