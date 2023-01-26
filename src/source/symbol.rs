use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::source::filepos::FileRange;

#[derive(EnumIter)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ParenthesisType {
    // ()
    Rounded,
    // {}
    Curly,
    // <>
    Angle,
    // []
    Square,
}

impl ParenthesisType {
    pub fn getCharacters(&self) -> [char; 2] {
        return match self {
            ParenthesisType::Rounded => ['(', ')'],
            ParenthesisType::Curly => ['{', '}'],
            ParenthesisType::Angle => ['<', '>'],
            ParenthesisType::Square => ['[', ']'],
        };
    }

    pub fn getOpening(&self) -> char {
        return self.getCharacters()[0];
    }

    pub fn getClosing(&self) -> char {
        return self.getCharacters()[1];
    }
}

#[derive(EnumIter)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum QuoteType {
    Double,
    Single,
}

impl QuoteType {
    pub fn getCharacter(&self) -> char {
        return match self {
            QuoteType::Double => '"',
            QuoteType::Single => '\'',
        };
    }
}

#[derive(EnumIter)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operator {
    // ++
    Increment,
    // --
    Decrement,
    // !
    Not,
    // ?
    ErrorPropagation,
    // as type
    Cast,
    // +
    Plus,
    // -
    Minus,
    // *
    Mult,
    // /
    Div,
    // %
    Mod,
    // +=
    PlusAssign,
    // -=
    MinusAssign,
    // *=
    MultAssign,
    // /=
    DivAssign,
    // %=
    ModAssign,
    // and
    And,
    // or
    Or,
    // >
    Greater,
    // <
    Less,
    // >=
    GreaterEq,
    // <=
    LessEq,
    // ==
    CompareEq,
    // !=
    CompareNotEq,
    // =
    AssignEq,
}

impl Operator {
    pub fn getPrecedence(&self) -> i32 {
        return match self {
            // 0
            Operator::AssignEq | Operator::PlusAssign | Operator::MinusAssign | Operator::MultAssign | Operator::DivAssign | Operator::ModAssign => 0,
            // 1
            // 2
            Operator::CompareEq | Operator::CompareNotEq | Operator::Greater | Operator::GreaterEq | Operator::Less | Operator::LessEq => 2,
            // 3
            Operator::And | Operator::Or => 3,
            // 4
            Operator::Plus | Operator::Minus => 4,
            // 5
            Operator::Mult | Operator::Div | Operator::Mod => 5,
            // 8
            Operator::Cast => 8,
            // 10
            Operator::Not | Operator::Increment | Operator::Decrement | Operator::ErrorPropagation => 10,
        };
    }

    pub fn getCharacters(&self) -> &'static str {
        return match self {
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::Not => "!",
            Operator::ErrorPropagation => "?",
            Operator::Cast => "as",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Mult => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::PlusAssign => "+=",
            Operator::MinusAssign => "-=",
            Operator::MultAssign => "*=",
            Operator::DivAssign => "/=",
            Operator::ModAssign => "%=",
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Greater => ">",
            Operator::Less => "<",
            Operator::GreaterEq => ">=",
            Operator::LessEq => "<=",
            Operator::CompareEq => "==",
            Operator::CompareNotEq => "!=",
            Operator::AssignEq => "=",
        };
    }

    pub fn isKeywordOperator(&self) -> bool {
        return match self {
            Operator::And | Operator::Or | Operator::Cast => true,
            Operator::Increment | Operator::Decrement | Operator::ErrorPropagation | Operator::Not | Operator::Plus | Operator::Minus | Operator::Mult | Operator::Div | Operator::Mod | Operator::PlusAssign | Operator::MinusAssign | Operator::MultAssign | Operator::DivAssign | Operator::ModAssign | Operator::Greater | Operator::Less | Operator::GreaterEq | Operator::LessEq | Operator::CompareEq | Operator::CompareNotEq | Operator::AssignEq => false,
        };
    }

    pub fn getKeywordOperators() -> &'static HashMap<&'static str, Operator> {
        static MAP: Lazy<HashMap<&'static str, Operator>> = Lazy::new(|| {
            let mut map = HashMap::new();
            for operator in Operator::iter() {
                if operator.isKeywordOperator() {
                    map.insert(operator.getCharacters(), operator);
                }
            }
            return map;
        });
        return &MAP;
    }

    pub fn getSymbolOperators() -> &'static HashMap<&'static str, Operator> {
        static MAP: Lazy<HashMap<&'static str, Operator>> = Lazy::new(|| {
            let mut map = HashMap::new();
            for operator in Operator::iter() {
                if !operator.isKeywordOperator() {
                    map.insert(operator.getCharacters(), operator);
                }
            }
            return map;
        });
        return &MAP;
    }
}

// EXCLUDES OPERATOR KEYWORDS (see Operator)
#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
#[strum(use_phf)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Keyword {
    // class
    Class,
    Static,
    Extends,
    // visibility
    Public,
    Private,
    // variable declaration
    Let,
    // control flow
    If,
    Else,
    While,
    For,
    Break,
    Return,
    // misc
    Enum,
}

#[derive(Debug)]
pub enum SymbolType {
    Identifier,
    SemiColan,
    Number,
    Keyword(Keyword),
    Comment(FileRange),
    CommaList(Vec<Symbol>),
    String(QuoteType, FileRange),
    Parenthesis(ParenthesisType, Vec<Symbol>),
    Operator(Operator),
}

#[derive(Debug)]
pub struct Symbol {
    symbolType: Box<SymbolType>,
    sourceRange: FileRange,
}

impl Symbol {
    pub fn new(symbolType: SymbolType, sourceRange: FileRange) -> Self {
        return Self {
            symbolType: Box::new(symbolType),
            sourceRange,
        };
    }

    pub fn getSymbolType(&self) -> &SymbolType {
        return self.symbolType.deref();
    }

    pub fn getSourceRange(&self) -> &FileRange {
        return &self.sourceRange;
    }
}