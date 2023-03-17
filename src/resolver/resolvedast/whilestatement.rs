use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::statement::{Statement, StatementType};

pub struct WhileStatement {
    pub condition: ResolvedExpr,
    pub statement: Statement
}

impl StatementType for WhileStatement {
}
