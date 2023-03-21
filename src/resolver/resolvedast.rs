use crate::resolver::resolvedast::statement::Statement;

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

#[derive(Debug)]
pub struct ResolvedAST {
    statementVec: Vec<Statement>,
}

impl ResolvedAST {
    pub fn new(statementVec: Vec<Statement>) -> Self {
        return Self {
            statementVec
        };
    }
}
