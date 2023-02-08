use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;

use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalfixed::LiteralFixed;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::operatorexpr::OperatorExpr;
use crate::ast::symbol::expr::parenthesisexpr::ParenthesisExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::ifstatement::IfSym;
use crate::ast::symbol::import::ImportSym;
use crate::module::modulepos::ModuleRange;

pub mod expr;
pub mod block;
pub mod function;
pub mod import;
pub mod classdefinition;
pub mod looptype;
pub mod breaksym;
pub mod ifstatement;

pub trait SymbolType {
    fn getRange(&self) -> &ModuleRange;
}

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum Symbol {
    // common symbols
    Block(BlockSym),
    Break(BreakSym),
    ClassDefinition(ClassDefinitionSym),
    FunctionDefinition(FunctionDefinitionSym),
    IfSym(IfSym),
    ImportSym(ImportSym),
    // expressions
    FunctionCall(FunctionCallExpr),
    Operator(OperatorExpr),
    Parenthesis(ParenthesisExpr),
    VariableDeclaration(VariableDeclarationExpr),
    Variable(VariableExpr),
    //  literal
    LiteralArray(LiteralArray),
    LiteralBool(LiteralBool),
    LiteralChar(LiteralChar),
    LiteralFixed(LiteralFixed),
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
            Symbol::LiteralFixed(symbol) => symbol,
            Symbol::LiteralInteger(symbol) => symbol,
            Symbol::LiteralString(symbol) => symbol,
            Symbol::LiteralVoid(symbol) => symbol,
            Symbol::LiteralTuple(symbol) => symbol
        };
    }

    pub fn getExpression(&self) -> Option<&dyn ExprType> {
        return match self {
            Symbol::Block(_) |
            Symbol::Break(_) |
            Symbol::ClassDefinition(_) |
            Symbol::FunctionDefinition(_) |
            Symbol::IfSym(_) |
            Symbol::ImportSym(_) => None,
            Symbol::FunctionCall(symbol) => Some(symbol),
            Symbol::Operator(symbol) => Some(symbol),
            Symbol::Parenthesis(symbol) => Some(symbol),
            Symbol::VariableDeclaration(symbol) => Some(symbol),
            Symbol::Variable(symbol) => Some(symbol),
            Symbol::LiteralArray(symbol) => Some(symbol),
            Symbol::LiteralBool(symbol) => Some(symbol),
            Symbol::LiteralChar(symbol) => Some(symbol),
            Symbol::LiteralFixed(symbol) => Some(symbol),
            Symbol::LiteralInteger(symbol) => Some(symbol),
            Symbol::LiteralString(symbol) => Some(symbol),
            Symbol::LiteralVoid(symbol) => Some(symbol),
            Symbol::LiteralTuple(symbol) => Some(symbol)
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
            Symbol::Variable(_) => None,
            Symbol::LiteralArray(symbol) => Some(symbol),
            Symbol::LiteralBool(symbol) => Some(symbol),
            Symbol::LiteralChar(symbol) => Some(symbol),
            Symbol::LiteralFixed(symbol) => Some(symbol),
            Symbol::LiteralInteger(symbol) => Some(symbol),
            Symbol::LiteralString(symbol) => Some(symbol),
            Symbol::LiteralVoid(symbol) => Some(symbol),
            Symbol::LiteralTuple(symbol) => Some(symbol),
        };
    }
}