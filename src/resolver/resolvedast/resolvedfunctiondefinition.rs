use crate::resolver::function::Function;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
use crate::resolver::resolvedast::statement::StatementType;

#[derive(Debug)]
pub struct ResolvedFunctionDefinition {
    pub function: Function,
    pub parameterVecId: Vec<usize>,
    pub scope: ResolvedScope,
}

impl StatementType for ResolvedFunctionDefinition {}
