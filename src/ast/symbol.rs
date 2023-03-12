use std::fmt::Debug;

use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;

use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::continuesym::ContinueSym;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalfloat::LiteralFloat;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::operatorexpr::OperatorExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::ifstatement::IfSym;
use crate::ast::symbol::import::ImportSym;
use crate::ast::symbol::looptype::whileloop::WhileLoop;
use crate::ast::symbol::returnsym::ReturnSym;
use crate::module::modulepos::ModuleRange;

pub mod expr;
pub mod block;
pub mod function;
pub mod import;
pub mod classdefinition;
pub mod looptype;
pub mod breaksym;
pub mod ifstatement;
pub mod continuesym;
pub mod returnsym;

pub trait SymbolType: Debug {
    fn getRange(&self) -> &ModuleRange;
}

#[derive(Debug)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum Symbol {
    // common symbols
    Block(BlockSym),
    // control flow
    Break(BreakSym),
    Continue(ContinueSym),
    While(WhileLoop),
    // Loop(Loop),
    // For(ForLoop),
    Return(ReturnSym),
    IfSym(IfSym),
    // structures
    ClassDefinition(ClassDefinitionSym),
    FunctionDefinition(FunctionDefinitionSym),
    ImportSym(ImportSym),
    // expressions
    FunctionCall(FunctionCallExpr),
    Operator(OperatorExpr),
    VariableDeclaration(VariableDeclarationExpr),
    Variable(VariableExpr),
    //  literal
    LiteralArray(LiteralArray),
    LiteralBool(LiteralBool),
    LiteralChar(LiteralChar),
    LiteralFloat(LiteralFloat),
    LiteralInteger(LiteralInteger),
    LiteralString(LiteralString),
    LiteralVoid(LiteralVoid),
    LiteralTuple(LiteralTuple),
}

impl Symbol {
    pub fn getSymbolType(&self) -> &dyn SymbolType {
        return match self {
            Symbol::Block(symbol) => symbol,
            Symbol::Break(symbol) => symbol,
            Symbol::ClassDefinition(symbol) => symbol,
            Symbol::FunctionDefinition(symbol) => symbol,
            Symbol::IfSym(symbol) => symbol,
            Symbol::ImportSym(symbol) => symbol,
            Symbol::FunctionCall(symbol) => symbol,
            Symbol::Operator(symbol) => symbol,
            Symbol::VariableDeclaration(symbol) => symbol,
            Symbol::Variable(symbol) => symbol,
            Symbol::LiteralArray(symbol) => symbol,
            Symbol::LiteralBool(symbol) => symbol,
            Symbol::LiteralChar(symbol) => symbol,
            Symbol::LiteralFloat(symbol) => symbol,
            Symbol::LiteralInteger(symbol) => symbol,
            Symbol::LiteralString(symbol) => symbol,
            Symbol::LiteralVoid(symbol) => symbol,
            Symbol::LiteralTuple(symbol) => symbol,
            Symbol::Continue(symbol) => symbol,
            Symbol::While(symbol) => symbol,
            // Symbol::Loop(symbol) => symbol,
            // Symbol::For(symbol) => symbol,
            Symbol::Return(symbol) => symbol,
        };
    }

    pub fn getExpression(&self) -> Option<&dyn ExprType> {
        return match self {
            Symbol::Block(_) |
            Symbol::Break(_) |
            Symbol::ClassDefinition(_) |
            Symbol::FunctionDefinition(_) |
            Symbol::IfSym(_) |
            Symbol::Continue(_) |
            Symbol::While(_) |
            // Symbol::Loop(_) |
            // Symbol::For(_) |
            Symbol::Return(_) |
            Symbol::ImportSym(_) => None,
            Symbol::FunctionCall(symbol) => Some(symbol),
            Symbol::Operator(symbol) => Some(symbol),
            Symbol::VariableDeclaration(symbol) => Some(symbol),
            Symbol::Variable(symbol) => Some(symbol),
            Symbol::LiteralArray(symbol) => Some(symbol),
            Symbol::LiteralBool(symbol) => Some(symbol),
            Symbol::LiteralChar(symbol) => Some(symbol),
            Symbol::LiteralFloat(symbol) => Some(symbol),
            Symbol::LiteralInteger(symbol) => Some(symbol),
            Symbol::LiteralString(symbol) => Some(symbol),
            Symbol::LiteralVoid(symbol) => Some(symbol),
            Symbol::LiteralTuple(symbol) => Some(symbol),
        };
    }

    pub fn toExpression(self) -> Option<Expr> {
        return match self {
            Symbol::Block(_) |
            Symbol::Break(_) |
            Symbol::ClassDefinition(_) |
            Symbol::FunctionDefinition(_) |
            Symbol::IfSym(_) |
            Symbol::Continue(_) |
            Symbol::While(_) |
            // Symbol::Loop(_) |
            // Symbol::For(_) |
            Symbol::Return(_) |
            Symbol::ImportSym(_) => None,
            Symbol::FunctionCall(symbol) => Some(Box::new(symbol)),
            Symbol::Operator(symbol) => Some(Box::new(symbol)),
            Symbol::VariableDeclaration(symbol) => Some(Box::new(symbol)),
            Symbol::Variable(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralArray(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralBool(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralChar(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralFloat(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralInteger(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralString(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralVoid(symbol) => Some(Box::new(symbol)),
            Symbol::LiteralTuple(symbol) => Some(Box::new(symbol)),
        };
    }

    pub fn getLiteral(&self) -> Option<&dyn LiteralType> {
        return match self {
            Symbol::Block(_) |
            Symbol::Break(_) |
            Symbol::ClassDefinition(_) |
            Symbol::FunctionDefinition(_) |
            Symbol::IfSym(_) |
            Symbol::ImportSym(_) |
            Symbol::FunctionCall(_) |
            Symbol::Operator(_) |
            Symbol::VariableDeclaration(_) |
            Symbol::Continue(_) |
            Symbol::While(_) |
            // Symbol::Loop(_) |
            // Symbol::For(_) |
            Symbol::Return(_) |
            Symbol::Variable(_) => None,
            Symbol::LiteralArray(symbol) => Some(symbol),
            Symbol::LiteralBool(symbol) => Some(symbol),
            Symbol::LiteralChar(symbol) => Some(symbol),
            Symbol::LiteralFloat(symbol) => Some(symbol),
            Symbol::LiteralInteger(symbol) => Some(symbol),
            Symbol::LiteralString(symbol) => Some(symbol),
            Symbol::LiteralVoid(symbol) => Some(symbol),
            Symbol::LiteralTuple(symbol) => Some(symbol),
        };
    }
}
