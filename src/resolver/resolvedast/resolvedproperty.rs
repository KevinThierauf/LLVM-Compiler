use std::fmt::Debug;

use crate::resolver::resolvedast::resolvedexpr::{ResolvedExpr, ResolvedExprType};
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::typeinfo::{Type, TypeProperty};

#[derive(Debug)]
pub struct ResolvedProperty {
    pub value: ResolvedExpr,
    pub property: TypeProperty,
}

impl StatementType for ResolvedProperty {}

impl ResolvedExprType for ResolvedProperty {
    fn getExpressionType(&self) -> Type {
        return self.property.ty.to_owned();
    }

    fn isAssignable(&self) -> bool {
        return true;
    }
}
