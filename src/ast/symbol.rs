use std::fmt::Debug;

use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;

use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::continuesym::ContinueSym;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalFloat::LiteralFloat;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::memberaccessexpr::MemberAccessExpr;
use crate::ast::symbol::expr::methodcallexpr::MethodCallExpr;
use crate::ast::symbol::expr::operatorexpr::OperatorExpr;
use crate::ast::symbol::expr::parenthesisexpr::ParenthesisExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::ifstatement::IfSym;
use crate::ast::symbol::import::ImportSym;
use crate::ast::symbol::looptype::forloop::ForLoop;
use crate::ast::symbol::looptype::r#loop::Loop;
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
    Loop(Loop),
    For(ForLoop),
    Return(ReturnSym),
    IfSym(IfSym),
    // structures
    ClassDefinition(ClassDefinitionSym),
    FunctionDefinition(FunctionDefinitionSym),
    ImportSym(ImportSym),
    // expressions
    FunctionCall(FunctionCallExpr),
    Operator(OperatorExpr),
    Parenthesis(ParenthesisExpr),
    VariableDeclaration(VariableDeclarationExpr),
    Variable(VariableExpr),
    MemberAccess(MemberAccessExpr),
    MethodCall(MethodCallExpr),
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
            Symbol::Parenthesis(symbol) => symbol,
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
            Symbol::Loop(symbol) => symbol,
            Symbol::For(symbol) => symbol,
            Symbol::Return(symbol) => symbol,
            Symbol::MemberAccess(symbol) => symbol,
            Symbol::MethodCall(symbol) => symbol,
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
            Symbol::Loop(_) |
            Symbol::For(_) |
            Symbol::Return(_) |
            Symbol::ImportSym(_) => None,
            Symbol::FunctionCall(symbol) => Some(symbol),
            Symbol::Operator(symbol) => Some(symbol),
            Symbol::Parenthesis(symbol) => Some(symbol),
            Symbol::VariableDeclaration(symbol) => Some(symbol),
            Symbol::Variable(symbol) => Some(symbol),
            Symbol::MemberAccess(symbol) => Some(symbol),
            Symbol::MethodCall(symbol) => Some(symbol),
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
            Symbol::Parenthesis(_) |
            Symbol::VariableDeclaration(_) |
            Symbol::Continue(_) |
            Symbol::While(_) |
            Symbol::Loop(_) |
            Symbol::For(_) |
            Symbol::Return(_) |
            Symbol::MemberAccess(_) |
            Symbol::MethodCall(_) |
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
