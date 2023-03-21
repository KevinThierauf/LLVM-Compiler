use std::fmt::Debug;

use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalfloat::LiteralFloat;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::operatorexpr::OperatorExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::module::modulepos::ModuleRange;

pub mod operatorexpr;
pub mod functioncall;
pub mod variableexpr;
pub mod literal;
pub mod variabledeclaration;

pub trait ExprType: 'static + SymbolType + Debug {
    fn getSymbolType(&self) -> &dyn SymbolType;
}

#[derive(Debug)]
pub enum Expr {
    FunctionCall(FunctionCallExpr),
    Operator(OperatorExpr),
    VariableDeclaration(VariableDeclarationExpr),
    Variable(VariableExpr),
    LiteralArray(LiteralArray),
    LiteralBool(LiteralBool),
    LiteralChar(LiteralChar),
    LiteralFloat(LiteralFloat),
    LiteralInteger(LiteralInteger),
    LiteralString(LiteralString),
    LiteralVoid(LiteralVoid),
    LiteralTuple(LiteralTuple),
}

impl Expr {
    pub fn toSymbol(self) -> Symbol {
        return Symbol::Expr(self);
    }

    pub fn getRange(&self) -> &ModuleRange {
        return self.getExprType().getRange();
    }

    pub fn getExprType(&self) -> &dyn ExprType {
        return match self {
            Expr::FunctionCall(v) => v,
            Expr::Operator(v) => v,
            Expr::VariableDeclaration(v) => v,
            Expr::Variable(v) => v,
            Expr::LiteralArray(v) => v,
            Expr::LiteralBool(v) => v,
            Expr::LiteralChar(v) => v,
            Expr::LiteralFloat(v) => v,
            Expr::LiteralInteger(v) => v,
            Expr::LiteralString(v) => v,
            Expr::LiteralVoid(v) => v,
            Expr::LiteralTuple(v) => v,
        }
    }
}
