use crate::resolver::resolvedast::ifstatement::IfStatement;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::whilestatement::WhileStatement;

pub trait StatementType {
}

pub enum Statement {
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Expr(ResolvedExpr),
}
