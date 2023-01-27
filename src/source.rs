use crate::source::filepos::SourceFile;
use crate::source::parser::{ParseError, Parser};
use crate::source::token::Token;

pub mod parser;
pub mod filepos;
pub mod token;

pub fn parseSource(source: SourceFile) -> Result<Vec<Token>, ParseError> {
    return Parser::new(source).parse();
}
