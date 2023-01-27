use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
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
    // .
    Dot,
    // ..
    Range,
    // ...
    Ellipsis,
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
            // assignment
            Operator::AssignEq | Operator::PlusAssign | Operator::MinusAssign | Operator::MultAssign | Operator::DivAssign | Operator::ModAssign => 0,
            // boolean logic
            Operator::And | Operator::Or => 2,
            // comparison
            Operator::CompareEq | Operator::CompareNotEq | Operator::Greater | Operator::GreaterEq | Operator::Less | Operator::LessEq => 3,
            Operator::Range => 4,
            Operator::Plus | Operator::Minus => 5,
            Operator::Mult | Operator::Div | Operator::Mod => 6,
            Operator::ErrorPropagation | Operator::Dot | Operator::Cast | Operator::Not | Operator::Increment | Operator::Decrement => 10,
            Operator::Ellipsis => 20,
        };
    }

    pub fn getCharacters(&self) -> &'static str {
        return match self {
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::Not => "!",
            Operator::Dot => ".",
            Operator::Range => "..",
            Operator::Ellipsis => "...",
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
            Operator::Increment | Operator::Decrement | Operator::Not | Operator::Dot | Operator::Range | Operator::Ellipsis | Operator::ErrorPropagation | Operator::Plus | Operator::Minus | Operator::Mult | Operator::Div | Operator::Mod | Operator::PlusAssign | Operator::MinusAssign | Operator::MultAssign | Operator::DivAssign | Operator::ModAssign | Operator::Greater | Operator::Less | Operator::GreaterEq | Operator::LessEq | Operator::CompareEq | Operator::CompareNotEq | Operator::AssignEq => false,
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

    pub fn getTokenOperators() -> &'static HashMap<&'static str, Operator> {
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
pub enum TokenType {
    Identifier,
    SemiColan,
    Number,
    Keyword(Keyword),
    Comment(FileRange),
    CommaList(Vec<Vec<Token>>),
    String(QuoteType, FileRange),
    Parenthesis(ParenthesisType, Vec<Token>),
    Operator(Operator),
}

pub struct Token {
    tokenType: Box<TokenType>,
    sourceRange: FileRange,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&format!("{:?}", self.tokenType));
    }
}

impl Token {
    pub fn new(tokenType: TokenType, sourceRange: FileRange) -> Self {
        return Self {
            tokenType: Box::new(tokenType),
            sourceRange,
        };
    }

    pub fn getTokenType(&self) -> &TokenType {
        return self.tokenType.deref();
    }

    pub fn getSourceRange(&self) -> &FileRange {
        return &self.sourceRange;
    }
}