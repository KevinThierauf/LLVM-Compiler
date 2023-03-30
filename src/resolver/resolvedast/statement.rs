use std::fmt::Debug;

use crate::resolver::resolvedast::ifstatement::IfStatement;
use crate::resolver::resolvedast::printstatement::PrintStatement;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedfunctiondefinition::ResolvedFunctionDefinition;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
use crate::resolver::resolvedast::returnstatement::ReturnStatement;
use crate::resolver::resolvedast::whilestatement::WhileStatement;

pub trait StatementType: Debug {}

#[derive(Debug)]
pub enum Statement {
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Return(ReturnStatement),
    Expr(ResolvedExpr),
    Print(PrintStatement),
    FunctionDefinition(ResolvedFunctionDefinition),
    Scope(ResolvedScope),
    Multiple(Vec<Statement>),
}
