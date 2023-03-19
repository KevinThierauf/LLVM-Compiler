use std::ops::Deref;
use crate::resolver::resolvedast::functioncall::FunctionCall;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::typeinfo::Type;

pub trait ResolvedExprType: StatementType {
    fn getExpressionType(&self) -> Type;
}

pub enum ResolvedExpr {
    Operator(Box<ResolvedOperator>),
    FunctionCall(Box<FunctionCall>),
    VariableDeclaration(VariableDeclare),
}

impl ResolvedExpr {
    pub fn getResolvedExprType(&self) -> &dyn ResolvedExprType {
        return match self {
            ResolvedExpr::Operator(v) => v.deref(),
            ResolvedExpr::FunctionCall(v) => v.deref(),
            ResolvedExpr::VariableDeclaration(v) => v,
        };
    }

    pub fn getExpressionType(&self) -> Type {
        return self.getResolvedExprType().getExpressionType();
    }
}
