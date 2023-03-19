pub mod functioncall;
pub mod ifstatement;
pub mod resolvedexpr;
pub mod statement;
pub mod whilestatement;
pub mod variabledeclare;
pub mod returnstatement;
pub mod resolvedoperator;

#[derive(Debug)]
pub struct ResolvedAST {}

impl ResolvedAST {
    pub fn new() -> Self {
        return Self {
        };
    }
}
