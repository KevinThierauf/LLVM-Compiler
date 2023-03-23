use crate::resolver::resolvedast::resolvedscope::ResolvedScope;

pub mod functioncall;
pub mod ifstatement;
pub mod resolvedexpr;
pub mod statement;
pub mod whilestatement;
pub mod variabledeclare;
pub mod returnstatement;
pub mod resolvedoperator;
pub mod resolvedvariable;
pub mod constructorcall;
pub mod resolvedproperty;
pub mod resolvedfunctiondefinition;
pub mod resolvedscope;

#[derive(Debug)]
pub struct ResolvedAST {
    resolved: ResolvedScope
}

impl ResolvedAST {
    pub fn new(resolved: ResolvedScope) -> Self {
        return Self {
            resolved
        };
    }

    pub fn take(self) -> ResolvedScope {
        return self.resolved;
    }
}
