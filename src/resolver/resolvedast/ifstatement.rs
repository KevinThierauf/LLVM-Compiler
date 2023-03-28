use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::statement::{Statement, StatementType};

#[derive(Debug)]
pub struct IfStatement {
    pub condition: ResolvedExpr,
    pub statement: Statement,
    pub elseStatement: Option<Statement>,
}

impl StatementType for IfStatement {}
