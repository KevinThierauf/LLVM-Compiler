use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::statement::StatementType;

#[derive(Debug)]
pub struct PrintStatement {
    pub value: ResolvedExpr,
}

impl StatementType for PrintStatement {
}
