use std::mem::swap;
use std::ops::Range;
use std::str::FromStr;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

use crate::source::filepos::{FilePos, FileRange, SourceFile};
use crate::source::parser::Token::*;
use crate::source::symbol::{Keyword, Operator, ParenthesisType, QuoteType, Symbol, SymbolType};

pub fn parse(source: SourceFile) -> Result<Vec<Symbol>, ParseError> {
    return Parser::new(source).parse();
}

fn isDigit(c: char) -> bool {
    return match c {
        '0'..='9' => true,
        _ => false,
    };
}

fn isIdentifierCharacter(c: char) -> bool {
    return isInnerIdentifierCharacter(c) || isOuterIdentifierCharacter(c);
}

pub fn isOuterIdentifierCharacter(c: char) -> bool {
    return match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    };
}

pub fn isInnerIdentifierCharacter(c: char) -> bool {
    return isDigit(c);
}

pub fn isWhitespaceChar(c: char) -> bool {
    return c == ' ' || c == '\t' || isEndOfLine(c);
}

pub fn isEndOfLine(c: char) -> bool {
    return c == '\n' || c == '\r';
}

pub fn isQuoteChar(c: char) -> Option<QuoteType> {
    for quote in QuoteType::iter() {
        if c == quote.getCharacter() {
            return Some(quote);
        }
    }
    return None;
}

pub fn isOpenParenthesis(c: char) -> Option<ParenthesisType> {
    for parenthesis in ParenthesisType::iter() {
        if c == parenthesis.getOpening() {
            return Some(parenthesis);
        }
    }
    return None;
}

pub fn isCloseParenthesis(c: char) -> Option<ParenthesisType> {
    for parenthesis in ParenthesisType::iter() {
        if c == parenthesis.getClosing() {
            return Some(parenthesis);
        }
    }
    return None;
}

pub fn isSymbolOperatorChar(c: char) -> bool {
    return match c {
        '+' | '-' | '!' | '?' | '*' | '/' | '%' | '=' | '<' | '>' => true,
        _ => false,
    };
}

pub fn getSymbolOperator(string: &str) -> Option<Operator> {
    return Operator::getSymbolOperators().get(string).map(|v| *v);
}

pub fn isNumberChar(c: char) -> bool {
    return isInnerNumberChar(c) || isOuterNumberChar(c);
}

pub fn isOuterNumberChar(c: char) -> bool {
    return match c {
        '0'..='9' | '.' => true,
        _ => false,
    };
}

// e.g. 0x12
fn isInnerNumberChar(c: char) -> bool {
    return match c {
        'x' | 'b' => true,
        _ => false,
    };
}

pub fn getKeyword(string: &str) -> Option<Keyword> {
    return Keyword::from_str(string).ok();
}

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
        return format!("parser error: \"{}\"\n at {}\n \"{}\"", self.errorMessage, self.fileRange, relevantSource);
    }
}

#[derive(EnumCount, EnumString, EnumIter, FromPrimitive)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
enum Token {
    Word = 0,
    Number,
    Operator,
}

impl Token {
    fn getBitMask(&self) -> u32 {
        return 1 << *self as u32;
    }
}

struct TokenOption {
    mask: u32,
}

enum TokenError {
    Invalid,
    Ambiguous(Vec<Token>),
}

impl TokenOption {
    fn new() -> Self {
        return Self {
            mask: !0,
        };
    }

    fn fromTokens(tokens: &[Token]) -> Self {
        let mut mask = 0;
        for token in tokens {
            mask += token.getBitMask();
        }
        return Self {
            mask
        };
    }

    fn setExclusiveOptions(&mut self, tokens: &[Token]) {
        self.mask &= TokenOption::fromTokens(tokens).mask;
    }

    fn isExactlyOneOf(&self, tokens: &[Token]) -> bool {
        return if let Ok(token) = self.getToken() {
            tokens.contains(&token)
        } else {
            false
        };
    }

    fn isOptionSet(&self, token: Token) -> bool {
        return self.mask & token.getBitMask() != 0;
    }

    fn getOptions(&self) -> Vec<Token> {
        let mut vec = Vec::new();
        for token in Token::iter() {
            if self.isOptionSet(token) {
                vec.push(token);
            }
        }
        return vec;
    }

    fn getOptionLength(&self) -> u32 {
        return self.mask.count_ones();
    }

    fn getToken(&self) -> Result<Token, TokenError> {
        match self.getOptionLength() {
            0 => Err(TokenError::Invalid),
            1 => Ok(Token::from_u32(self.mask.ilog2()).expect("invalid mask")),
            _ => Err(TokenError::Ambiguous(self.getOptions())),
        }
    }
}

struct Parenthesis {
    openingIndex: usize,
    // symbol vec for symbols next to this set of parenthesis
    // once parenthesis is closed, parsing will resume to externalSymbolVec
    //  (parenthesis symbol will end up being added to externalSymbolVec)
    parentSymbolVec: ParserSymbolVec,
    parenthesisType: ParenthesisType,
}

struct ParenthesisSet {
    parenthesisVec: Vec<Parenthesis>,
}

impl ParenthesisSet {
    fn new() -> Self {
        return Self {
            parenthesisVec: Vec::new(),
        };
    }

    fn push(&mut self, openingIndex: usize, symbolVec: &mut ParserSymbolVec, parenthesisType: ParenthesisType) {
        self.parenthesisVec.push(Parenthesis {
            openingIndex,
            parentSymbolVec: symbolVec.makeChild(openingIndex),
            parenthesisType,
        });
    }

    fn pop(&mut self, parenthesisType: ParenthesisType) -> Result<(ParserSymbolVec, usize), Option<Parenthesis>> {
        return if let Some(parenthesis) = self.parenthesisVec.pop() {
            if parenthesisType == parenthesis.parenthesisType {
                return Ok((parenthesis.parentSymbolVec, parenthesis.openingIndex));
            } else {
                Err(Some(parenthesis))
            }
        } else {
            Err(None)
        };
    }
}

struct ParserSymbolVec {
    symbolVec: Vec<Symbol>,
    startIndex: FilePos,
    nextSymbolComma: bool,
    commaSymbolVec: Vec<Symbol>,
}

impl ParserSymbolVec {
    fn new(startIndex: FilePos) -> Self {
        return Self {
            symbolVec: Vec::new(),
            startIndex,
            nextSymbolComma: false,
            commaSymbolVec: Vec::new(),
        };
    }

    fn foldCommaVec(&mut self, nextCharacterIndex: usize) {
        // currently unused ending ',' is allowed and ignored
        // e.g. 1, 2, 3,
        // vs   1, 2, 3
        match self.commaSymbolVec.len() {
            0 => {}
            1 => self.symbolVec.append(&mut self.commaSymbolVec),
            _ => {
                let startIndex = self.startIndex.getIndex();
                debug_assert!(startIndex <= nextCharacterIndex);
                let mut commaSymbolVec = Vec::new();
                commaSymbolVec.append(&mut self.commaSymbolVec);
                self.symbolVec.push(Symbol::new(
                    SymbolType::CommaList(commaSymbolVec),
                    FileRange::new(self.startIndex.to_owned(), nextCharacterIndex - startIndex),
                ));
            }
        };
    }

    fn addSymbolRange(&mut self, symbolType: SymbolType, range: FileRange) {
        if self.nextSymbolComma {
            self.nextSymbolComma = false;
        } else {
            self.foldCommaVec(range.getEndIndex());
        }
        self.commaSymbolVec.push(Symbol::new(symbolType, range));
    }

    fn setCommaNext(&mut self) {
        assert!(!self.nextSymbolComma);
        self.nextSymbolComma = true;
    }

    fn takeSymbolVec(mut self, nextCharacterIndex: usize) -> Vec<Symbol> {
        debug_assert!(self.commaSymbolVec.is_empty());
        self.foldCommaVec(nextCharacterIndex);
        return self.symbolVec;
    }

    // convert to child, returns parent
    #[must_use]
    fn makeChild(&mut self, nextCharacterIndex: usize) -> ParserSymbolVec {
        let mut other = ParserSymbolVec::new(FilePos::new(self.startIndex.getSourceFile().to_owned(), nextCharacterIndex));
        swap(self, &mut other);
        return other;
    }

    // takes parent, convert to parent, returns vec of parsed symbols
    #[must_use]
    fn makeParent(&mut self, nextCharacterIndex: usize, parent: ParserSymbolVec) -> Vec<Symbol> {
        let mut child = parent;
        swap(self, &mut child);
        return child.takeSymbolVec(nextCharacterIndex);
    }
}

struct Parser {
    sourceFile: SourceFile,
    parserVec: ParserSymbolVec,
    parenthesisSet: ParenthesisSet,
    nextCharacterIndex: usize,
    lastSymbolStart: usize,
    tokenStart: usize,
    tokenOption: TokenOption,
}

impl Parser {
    fn new(sourceFile: SourceFile) -> Self {
        return Self {
            parserVec: ParserSymbolVec::new(FilePos::new(sourceFile.to_owned(), 0)),
            sourceFile,
            parenthesisSet: ParenthesisSet::new(),
            nextCharacterIndex: 0,
            tokenOption: TokenOption::new(),
            lastSymbolStart: 0,
            tokenStart: 0,
        };
    }

    fn getTokenRange(&self) -> Range<usize> {
        return self.tokenStart..self.nextCharacterIndex;
    }

    fn isFirstCharacterInToken(&self) -> bool {
        return self.tokenStart + 1 == self.nextCharacterIndex;
    }

    fn getNextChar(&mut self) -> Option<char> {
        let index = self.nextCharacterIndex;
        self.nextCharacterIndex += 1;
        return self.sourceFile.getSource().as_bytes().get(index).map(|v| *v as char);
    }

    fn peekNextChar(&self, value: char) -> bool {
        return if self.nextCharacterIndex >= self.sourceFile.getSource().len() { false } else { self.sourceFile.getSource().as_bytes()[self.nextCharacterIndex] as char == value };
    }

    fn getTokenSource(&self) -> &str {
        return &self.sourceFile.getSource()[self.getTokenRange()];
    }

    fn getFilePos(&self, index: usize) -> FilePos {
        debug_assert!(index <= self.sourceFile.getLength());
        return FilePos::new(self.sourceFile.to_owned(), index);
    }

    fn getFileRange(&self, range: Range<usize>) -> FileRange {
        debug_assert!(range.end <= self.sourceFile.getLength());
        debug_assert!(range.start <= range.end);
        return FileRange::new(self.getFilePos(range.start), range.end - range.start);
    }

    fn getErrorRange(&self, range: Range<usize>, message: String) -> ParseError {
        return ParseError::new(self.getFileRange(range), message);
    }

    fn getErrorTokenRange(&self, message: String) -> ParseError {
        return self.getErrorRange(self.getTokenRange(), message);
    }

    fn addSymbol(&mut self, symbolType: SymbolType) {
        self.parserVec.addSymbolRange(symbolType, self.getFileRange(self.lastSymbolStart..self.nextCharacterIndex));
        self.lastSymbolStart = self.nextCharacterIndex;
    }

    fn addTokenExcludeLastChar(&mut self, skipCurrent: bool) -> Result<(), ParseError> {
        self.nextCharacterIndex -= 1;
        self.addCurrentToken()?;
        self.nextCharacterIndex += 1;
        if skipCurrent {
            self.tokenStart = self.nextCharacterIndex;
        }
        return Ok(());
    }

    fn addCurrentToken(&mut self) -> Result<(), ParseError> {
        if self.getTokenRange().is_empty() {
            return Ok(());
        }

        match self.tokenOption.getToken() {
            Ok(token) => {
                self.addToken(token)?;
                Ok(())
            }
            Err(err) => return Err(self.getErrorTokenRange(match err {
                TokenError::Invalid => format!("unable to match expr to token: no matches found"),
                TokenError::Ambiguous(vec) => format!("expr matches multiple possible tokens: {:#?}", vec)
            })),
        }
    }

    fn addToken(&mut self, token: Token) -> Result<(), ParseError> {
        self.tokenOption = TokenOption::new();
        match token {
            Word => {
                let tokenSource = self.getTokenSource();
                if let Some(keyword) = getKeyword(tokenSource) {
                    self.addSymbol(SymbolType::Keyword(keyword));
                } else if let Some(&operator) = Operator::getKeywordOperators().get(tokenSource) {
                    self.addSymbol(SymbolType::Operator(operator));
                } else {
                    self.addSymbol(SymbolType::Identifier);
                }
            }
            Number => {
                self.addSymbol(SymbolType::Number);
            }
            Operator => {
                let tokenSource = self.getTokenSource();
                if let Some(operator) = getSymbolOperator(tokenSource) {
                    self.addSymbol(SymbolType::Operator(operator));
                } else {
                    // todo - support multiple adjacent operators
                    //  such as !!
                    return Err(self.getErrorTokenRange(format!("unknown operator {tokenSource}")));
                }
            }
        }
        self.tokenStart = self.nextCharacterIndex;
        return Ok(());
    }

    fn parse(mut self) -> Result<Vec<Symbol>, ParseError> {
        let mut lastCharacterIndex = self.nextCharacterIndex;
        while let Some(character) = self.getNextChar() {
            match character {
                _ if isWhitespaceChar(character) => {
                    self.addTokenExcludeLastChar(true)?;
                }
                _ if isIdentifierCharacter(character)
                    || isNumberChar(character) => {
                    fn setExclusiveOptions(tokenOption: &mut TokenOption, word: bool, number: bool) {
                        let options = match (word, number) {
                            (true, true) => [Word, Number].as_slice(),
                            (true, false) => &[Word],
                            (false, true) => &[Number],
                            (false, false) => &[]
                        };

                        tokenOption.setExclusiveOptions(options);
                    }

                    if self.isFirstCharacterInToken() {
                        setExclusiveOptions(&mut self.tokenOption, isOuterIdentifierCharacter(character), isOuterNumberChar(character))
                    } else {
                        setExclusiveOptions(&mut self.tokenOption, isInnerIdentifierCharacter(character) || isOuterIdentifierCharacter(character), isInnerNumberChar(character) || isOuterNumberChar(character));
                    }
                }
                ',' => {
                    self.addTokenExcludeLastChar(true)?;
                    if self.parserVec.nextSymbolComma {
                        return Err(self.getErrorTokenRange(format!("multiple unexpected commas")));
                    }
                    self.parserVec.setCommaNext();
                }
                '/' if self.peekNextChar('/') => {
                    // line comment
                    let commentStart = self.tokenStart;
                    while let Some(character) = self.getNextChar() {
                        if isEndOfLine(character) {
                            break;
                        }
                    }
                    self.addSymbol(SymbolType::Comment(self.getFileRange(commentStart..self.nextCharacterIndex)));
                }
                '/' if self.peekNextChar('*') => {
                    // multi-line comment
                    let commentStart = self.tokenStart;
                    const FIRST: char = '*';
                    const SECOND: char = '/';
                    let mut expected = '/';
                    loop {
                        if let Some(character) = self.getNextChar() {
                            if character == expected {
                                if expected == FIRST {
                                    expected = SECOND;
                                } else {
                                    debug_assert_eq!(expected, SECOND);
                                    break;
                                }
                            } else {
                                expected = FIRST;
                            }
                        } else {
                            return Err(self.getErrorTokenRange(format!("reached end of file before finding closing */")));
                        }
                    }
                    debug_assert!(self.nextCharacterIndex >= 4); // enough space for /**/
                    self.addSymbol(SymbolType::Comment(self.getFileRange(commentStart..self.nextCharacterIndex - 2)));
                }
                _ if isSymbolOperatorChar(character) => {
                    if self.tokenOption.isExactlyOneOf(&[Word, Number]) {
                        self.addTokenExcludeLastChar(false)?;
                    }
                    self.tokenOption.setExclusiveOptions(&[
                        Operator
                    ]);
                }
                // if let guards are currently unstable, call isQuoteChar/isOpenParenthesis/isCloseParenthesis twice with same parameters
                _ if isQuoteChar(character).is_some() => {
                    let quoteType = isQuoteChar(character).unwrap();
                    let startIndex = self.nextCharacterIndex;
                    loop {
                        const ESCAPE_CHAR: char = '\\';
                        let closeQuote: char = quoteType.getCharacter();
                        if let Some(character) = self.getNextChar() {
                            if character == closeQuote {
                                break;
                            } else if character == ESCAPE_CHAR {
                                // skip next character
                                self.getNextChar();
                            }
                        } else {
                            return Err(self.getErrorTokenRange(format!("missing closing {} quote", closeQuote)));
                        }
                    }

                    self.addSymbol(SymbolType::String(quoteType, self.getFileRange(startIndex..lastCharacterIndex)));
                }
                _ if isOpenParenthesis(character).is_some() => {
                    self.parenthesisSet.push(lastCharacterIndex, &mut self.parserVec, isOpenParenthesis(character).unwrap());
                }
                c if isCloseParenthesis(character).is_some() => {
                    let parenthesisType = isCloseParenthesis(character).unwrap();
                    match self.parenthesisSet.pop(parenthesisType) {
                        Ok((parentSymbolVec, openIndex)) => {
                            let symbolVec = self.parserVec.makeParent(self.nextCharacterIndex, parentSymbolVec);
                            self.parserVec.addSymbolRange(SymbolType::Parenthesis(parenthesisType, symbolVec), self.getFileRange(openIndex..self.nextCharacterIndex));
                        }
                        Err(err) => {
                            return Err(self.getErrorTokenRange(if let Some(parenthesis) = err {
                                format!("found closing '{c}' parenthesis, expected closing {}", parenthesis.parenthesisType.getClosing())
                            } else {
                                format!("unmatched closing '{c}' parenthesis")
                            }));
                        }
                    }
                }
                ';' => {
                    self.addTokenExcludeLastChar(false)?;
                    self.addSymbol(SymbolType::SemiColan);
                }
                _ => return Err(self.getErrorTokenRange(format!("unexpected character {character}")))
            }
            lastCharacterIndex = self.nextCharacterIndex;
        }
        self.addCurrentToken()?;

        if let Some(parenthesis) = self.parenthesisSet.parenthesisVec.last() {
            let index = parenthesis.openingIndex;
            return Err(self.getErrorRange(index..self.nextCharacterIndex, self.parenthesisSet.parenthesisVec.iter().map(
                |parenthesis| format!("unmatched opening {} parenthesis", parenthesis.parenthesisType.getOpening())
            ).collect::<Vec<String>>().join(", ")));
        }

        return Ok(self.parserVec.takeSymbolVec(self.nextCharacterIndex));
    }
}