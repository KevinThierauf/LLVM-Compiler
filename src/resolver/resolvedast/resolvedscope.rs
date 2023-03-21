use crate::resolver::resolvedast::statement::{Statement, StatementType};

#[derive(Debug)]
pub struct ResolvedScope {
    pub statementVec: Vec<Statement>
}

impl StatementType for ResolvedScope {}
