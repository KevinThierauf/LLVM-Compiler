use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::statement::StatementType;

#[derive(Debug)]
pub struct ReturnStatement {
    pub expr: Option<ResolvedExpr>
}

impl StatementType for ReturnStatement {}
