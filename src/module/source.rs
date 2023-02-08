use crate::module::source::filepos::{FileRange, SourceFile};
use crate::module::source::sourceparser::SourceParser;
use crate::module::source::token::Token;

pub mod sourceparser;
pub mod filepos;
pub mod token;

pub struct ParseError {
    fileRange: FileRange,
    errorMessage: String,
}

impl ParseError {
    pub fn new(fileRange: FileRange, errorMessage: String) -> Self {
        return Self {
            fileRange,
            errorMessage,
        };
    }

    pub fn getFileRange(&self) -> &FileRange {
        return &self.fileRange;
    }

    pub fn getError(&self) -> &String {
        return &self.errorMessage;
    }

    pub fn getDisplayMessage(&self) -> String {
        const MAX_SOURCE_DISPLAY_LENGTH: usize = 100;
        let mut relevantSource = self.fileRange.getSourceInRange();
        if relevantSource.len() > MAX_SOURCE_DISPLAY_LENGTH {
            relevantSource = &relevantSource[0..MAX_SOURCE_DISPLAY_LENGTH];
        }
        return format!("Parser error: \"{}\"\n at {}\n \"{}\"", self.errorMessage, self.fileRange, relevantSource);
    }
}

pub fn parseSource(source: SourceFile) -> Result<Vec<Token>, ParseError> {
    return SourceParser::new(source).parse();
}
