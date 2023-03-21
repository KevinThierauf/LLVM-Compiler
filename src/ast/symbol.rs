use std::fmt::Debug;

use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::continuesym::ContinueSym;
use crate::ast::symbol::expr::{Expr, ExprType};
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
    Expr(Expr),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SymbolDiscriminants {
    Block,
    Break,
    Continue,
    While,
    Return,
    IfSym,
    ClassDefinition,
    FunctionDefinition,
    ImportSym,
    FunctionCall,
    Operator,
    VariableDeclaration,
    Variable,
    LiteralArray,
    LiteralBool,
    LiteralChar,
    LiteralFloat,
    LiteralInteger,
    LiteralString,
    LiteralVoid,
    LiteralTuple,
}

impl<'a> From<&'a Symbol> for SymbolDiscriminants {
    fn from(value: &'a Symbol) -> Self {
        return match value {
            Symbol::Block(_) => SymbolDiscriminants::Block,
            Symbol::Break(_) => SymbolDiscriminants::Break,
            Symbol::Continue(_) => SymbolDiscriminants::Continue,
            Symbol::While(_) => SymbolDiscriminants::While,
            Symbol::Return(_) => SymbolDiscriminants::Return,
            Symbol::IfSym(_) => SymbolDiscriminants::IfSym,
            Symbol::ClassDefinition(_) => SymbolDiscriminants::ClassDefinition,
            Symbol::FunctionDefinition(_) => SymbolDiscriminants::FunctionDefinition,
            Symbol::ImportSym(_) => SymbolDiscriminants::ImportSym,
            Symbol::Expr(expr) => {
                match expr {
                    Expr::FunctionCall(_) => SymbolDiscriminants::FunctionCall,
                    Expr::Operator(_) => SymbolDiscriminants::Operator,
                    Expr::VariableDeclaration(_) => SymbolDiscriminants::VariableDeclaration,
                    Expr::Variable(_) => SymbolDiscriminants::Variable,
                    Expr::LiteralArray(_) => SymbolDiscriminants::LiteralArray,
                    Expr::LiteralBool(_) => SymbolDiscriminants::LiteralBool,
                    Expr::LiteralChar(_) => SymbolDiscriminants::LiteralChar,
                    Expr::LiteralFloat(_) => SymbolDiscriminants::LiteralFloat,
                    Expr::LiteralInteger(_) => SymbolDiscriminants::LiteralInteger,
                    Expr::LiteralString(_) => SymbolDiscriminants::LiteralString,
                    Expr::LiteralVoid(_) => SymbolDiscriminants::LiteralVoid,
                    Expr::LiteralTuple(_) => SymbolDiscriminants::LiteralTuple,
                }
            }
        };
    }
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
            Symbol::Continue(symbol) => symbol,
            Symbol::While(symbol) => symbol,
            // Symbol::Loop(symbol) => symbol,
            // Symbol::For(symbol) => symbol,
            Symbol::Return(symbol) => symbol,
            Symbol::Expr(expr) => expr.getExprType().getSymbolType(),
        };
    }

    pub fn getExpression(&self) -> Option<&dyn ExprType> {
        return if let Symbol::Expr(expr) = self {
            Some(expr.getExprType())
        } else {
            None
        }
    }

    pub fn toExpression(self) -> Option<Expr> {
        return if let Symbol::Expr(expr) = self {
            Some(expr)
        } else {
            None
        }
    }
}
