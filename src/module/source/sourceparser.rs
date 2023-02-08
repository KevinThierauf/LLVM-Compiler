use std::mem::swap;
use std::ops::Range;
use std::str::FromStr;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use crate::module::ParseError;
use crate::module::source::filepos::{FilePos, FileRange, SourceFile};
use crate::module::source::sourceparser::BasicToken::*;
use crate::module::source::token::{Keyword, Operator, ParenthesisType, QuoteType, Token, TokenType};

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

pub fn isTokenOperatorChar(c: char) -> bool {
    return match c {
        '.' | '+' | '-' | '!' | '?' | '*' | '/' | '%' | '=' | '<' | '>' => true,
        _ => false,
    };
}

pub fn getTokenOperator(string: &str) -> Option<Operator> {
    return Operator::getTokenOperators().get(string).map(|v| *v);
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

#[derive(EnumCount, EnumString, EnumIter, FromPrimitive)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
enum BasicToken {
    Word = 0,
    Number,
    Operator,
}

impl BasicToken {
    fn getBitMask(&self) -> u32 {
        return 1 << *self as u32;
    }
}

struct BasicTokenOption {
    mask: u32,
}

enum BasicTokenError {
    Invalid,
    Ambiguous(Vec<BasicToken>),
}

impl BasicTokenOption {
    fn new() -> Self {
        return Self {
            mask: !0,
        };
    }

    fn fromBasicTokens(basicTokens: &[BasicToken]) -> Self {
        let mut mask = 0;
        for basicToken in basicTokens {
            mask |= basicToken.getBitMask();
        }
        return Self {
            mask
        };
    }

    fn setExclusiveOptions(&mut self, basicTokens: &[BasicToken]) {
        self.mask &= BasicTokenOption::fromBasicTokens(basicTokens).mask;
    }

    fn setPotentialExclusiveOptions(&mut self, basicTokens: &[Option<BasicToken>]) {
        let mut compareMask = 0;
        for token in basicTokens {
            if let Some(token) = token {
                compareMask |= token.getBitMask();
            }
        }
        self.mask &= compareMask;
    }

    fn removeOption(&mut self, token: BasicToken) {
        self.mask &= !token.getBitMask();
    }

    fn isExactlyOneOf(&self, basicTokens: &[BasicToken]) -> bool {
        return if let Ok(basicToken) = self.getBasicToken() {
            basicTokens.contains(&basicToken)
        } else {
            false
        };
    }

    fn isOptionSet(&self, basicToken: BasicToken) -> bool {
        return self.mask & basicToken.getBitMask() != 0;
    }

    fn getOptions(&self) -> Vec<BasicToken> {
        let mut vec = Vec::new();
        for basicToken in BasicToken::iter() {
            if self.isOptionSet(basicToken) {
                vec.push(basicToken);
            }
        }
        return vec;
    }

    fn getOptionLength(&self) -> u32 {
        return self.mask.count_ones();
    }

    fn getBasicToken(&self) -> Result<BasicToken, BasicTokenError> {
        match self.getOptionLength() {
            0 => Err(BasicTokenError::Invalid),
            1 => Ok(BasicToken::from_u32(self.mask.ilog2()).expect("invalid mask")),
            _ => Err(BasicTokenError::Ambiguous(self.getOptions())),
        }
    }
}

struct Parenthesis {
    openingIndex: usize,
    // token vec for tokens next to this set of parenthesis
    // once parenthesis is closed, parsing will resume to externalTokenVec
    //  (parenthesis token will end up being added to externalTokenVec)
    parentTokenVec: ParserTokenVec,
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

    fn push(&mut self, openingIndex: usize, tokenVec: &mut ParserTokenVec, parenthesisType: ParenthesisType) {
        self.parenthesisVec.push(Parenthesis {
            openingIndex,
            parentTokenVec: tokenVec.makeChild(openingIndex),
            parenthesisType,
        });
    }

    fn pop(&mut self, parenthesisType: ParenthesisType) -> Result<(ParserTokenVec, usize), Option<Parenthesis>> {
        return if let Some(parenthesis) = self.parenthesisVec.pop() {
            if parenthesisType == parenthesis.parenthesisType {
                return Ok((parenthesis.parentTokenVec, parenthesis.openingIndex));
            } else {
                Err(Some(parenthesis))
            }
        } else {
            Err(None)
        };
    }
}

struct ParserTokenVec {
    tokenVec: Vec<Token>,
    startIndex: FilePos,
    currentCommaVec: Vec<Token>,
    commaTokenVec: Vec<Vec<Token>>,
}

impl ParserTokenVec {
    fn new(startIndex: FilePos) -> Self {
        return Self {
            tokenVec: Vec::new(),
            startIndex,
            currentCommaVec: Vec::new(),
            commaTokenVec: Vec::new(),
        };
    }

    fn foldCommaVec(&mut self, nextCharacterIndex: usize) {
        // currently unused ending ',' is allowed and ignored
        // e.g. 1, 2, 3,
        // vs   1, 2, 3
        if self.commaTokenVec.is_empty() {
            // no commas used
            self.tokenVec.append(&mut self.currentCommaVec);
            self.commaTokenVec.clear();
        } else {
            let startIndex = self.startIndex.getIndex();
            debug_assert!(startIndex <= nextCharacterIndex);
            let mut currentCommaVec = Vec::new();
            currentCommaVec.append(&mut self.currentCommaVec);

            let mut commaTokenVec = Vec::new();
            commaTokenVec.append(&mut self.commaTokenVec);
            commaTokenVec.push(currentCommaVec);

            self.tokenVec.push(Token::new(
                TokenType::CommaList(commaTokenVec),
                FileRange::new(self.startIndex.to_owned(), nextCharacterIndex - startIndex),
            ));
        }
        debug_assert!(self.commaTokenVec.is_empty());
        debug_assert!(self.currentCommaVec.is_empty());
    }

    fn addComma(&mut self) {
        let mut nextVec = Vec::new();
        swap(&mut nextVec, &mut self.currentCommaVec);
        self.commaTokenVec.push(nextVec);
    }

    fn addTokenRange(&mut self, tokenType: TokenType, range: FileRange) {
        self.currentCommaVec.push(Token::new(tokenType, range));
    }

    fn takeTokenVec(mut self, nextCharacterIndex: usize) -> Vec<Token> {
        self.foldCommaVec(nextCharacterIndex);
        return self.tokenVec;
    }

    // convert to child, returns parent
    #[must_use]
    fn makeChild(&mut self, nextCharacterIndex: usize) -> ParserTokenVec {
        let mut other = ParserTokenVec::new(FilePos::new(self.startIndex.getSourceFile().to_owned(), nextCharacterIndex));
        swap(self, &mut other);
        return other;
    }

    // takes parent, convert to parent, returns vec of parsed tokens
    #[must_use]
    fn makeParent(&mut self, nextCharacterIndex: usize, parent: ParserTokenVec) -> Vec<Token> {
        let mut child = parent;
        swap(self, &mut child);
        return child.takeTokenVec(nextCharacterIndex);
    }
}

pub(crate) struct SourceParser {
    sourceFile: SourceFile,
    parserVec: ParserTokenVec,
    parenthesisSet: ParenthesisSet,
    nextCharacterIndex: usize,
    lastTokenStart: usize,
    basicTokenStart: usize,
    basicTokenOption: BasicTokenOption,
}

impl SourceParser {
    pub(crate) fn new(sourceFile: SourceFile) -> Self {
        return Self {
            parserVec: ParserTokenVec::new(FilePos::new(sourceFile.to_owned(), 0)),
            sourceFile,
            parenthesisSet: ParenthesisSet::new(),
            nextCharacterIndex: 0,
            basicTokenOption: BasicTokenOption::new(),
            lastTokenStart: 0,
            basicTokenStart: 0,
        };
    }

    fn getBasicTokenRange(&self) -> Range<usize> {
        return self.basicTokenStart..self.nextCharacterIndex;
    }

    fn isFirstCharacterInBasicToken(&self) -> bool {
        return self.basicTokenStart + 1 == self.nextCharacterIndex;
    }

    fn getNextChar(&mut self) -> Option<char> {
        return self.sourceFile.getSource().as_bytes().get(self.nextCharacterIndex).map(|v| {
            self.nextCharacterIndex += 1;
            *v as char
        });
    }

    fn peekNextChar(&self, value: char) -> bool {
        return if self.nextCharacterIndex >= self.sourceFile.getSource().len() { false } else { self.sourceFile.getSource().as_bytes()[self.nextCharacterIndex] as char == value };
    }

    fn getBasicTokenSource(&self) -> &str {
        return &self.sourceFile.getSource()[self.getBasicTokenRange()];
    }

    fn getFilePos(&self, index: usize) -> FilePos {
        debug_assert!(index <= self.sourceFile.getLength());
        return FilePos::new(self.sourceFile.to_owned(), index);
    }

    fn getFileRange(&self, range: Range<usize>) -> FileRange {
        debug_assert!(range.end <= self.sourceFile.getLength(), "{} <= {}", range.end, self.sourceFile.getLength());
        debug_assert!(range.start <= range.end, "{} <= {}", range.start, range.end);
        return FileRange::new(self.getFilePos(range.start), range.end - range.start);
    }

    fn getErrorRange(&self, range: Range<usize>, message: String) -> ParseError {
        return ParseError::new(self.getFileRange(range), message);
    }

    fn getErrorBasicTokenRange(&self, message: String) -> ParseError {
        return self.getErrorRange(self.getBasicTokenRange(), message);
    }

    fn addToken(&mut self, tokenType: TokenType) {
        self.parserVec.addTokenRange(tokenType, self.getFileRange(self.lastTokenStart..self.nextCharacterIndex));
        self.resetBasicToken();
        self.lastTokenStart = self.nextCharacterIndex;
    }

    fn addBasicTokenExcludeLastChar(&mut self, skipCurrent: bool) -> Result<(), ParseError> {
        self.nextCharacterIndex -= 1;
        self.addCurrentBasicToken()?;
        self.nextCharacterIndex += 1;
        if skipCurrent {
            self.basicTokenStart = self.nextCharacterIndex;
        }
        return Ok(());
    }

    #[inline(always)]
    fn isBasicTokenComplete(&mut self, token: BasicToken) -> bool {
        return match token {
            Number => {
                let source = self.getBasicTokenSource().as_bytes();
                let lastChar = source[source.len() - 1] as char;
                !(isInnerNumberChar(lastChar) || lastChar == '.')
            }
            Word | Operator => true,
        };
    }

    fn addCurrentBasicToken(&mut self) -> Result<(), ParseError> {
        if self.getBasicTokenRange().is_empty() {
            return Ok(());
        }

        if self.basicTokenOption.isOptionSet(Number) && !self.isBasicTokenComplete(Number) {
            self.basicTokenOption.removeOption(Number);
        }

        match self.basicTokenOption.getBasicToken() {
            Ok(basicToken) => {
                self.addBasicToken(basicToken)?;
                Ok(())
            }
            Err(err) => return Err(self.getErrorBasicTokenRange(match err {
                BasicTokenError::Invalid => format!("unable to match expr to tokens: no matches found"),
                BasicTokenError::Ambiguous(vec) => format!("expr matches multiple possible tokens: {:#?}", vec)
            })),
        }
    }

    fn resetBasicToken(&mut self) {
        self.basicTokenOption = BasicTokenOption::new();
        self.basicTokenStart = self.nextCharacterIndex;
    }

    fn addBasicToken(&mut self, basicToken: BasicToken) -> Result<(), ParseError> {
        match basicToken {
            Word => {
                let basicTokenSource = self.getBasicTokenSource();
                if let Some(keyword) = getKeyword(basicTokenSource) {
                    self.addToken(TokenType::Keyword(keyword));
                } else if let Some(&operator) = Operator::getKeywordOperators().get(basicTokenSource) {
                    self.addToken(TokenType::Operator(operator));
                } else {
                    self.addToken(TokenType::Identifier);
                }
            }
            Number => {
                self.addToken(TokenType::Number);
            }
            Operator => {
                let basicTokenSource = self.getBasicTokenSource();
                if let Some(operator) = getTokenOperator(basicTokenSource) {
                    self.addToken(TokenType::Operator(operator));
                } else {
                    // todo - support multiple adjacent operators
                    //  such as !!
                    return Err(self.getErrorBasicTokenRange(format!("unknown operator {basicTokenSource}")));
                }
            }
        }
        debug_assert_eq!(self.basicTokenStart, self.nextCharacterIndex, "basicToken not reset");
        return Ok(());
    }

    pub(crate) fn parse(mut self) -> Result<Vec<Token>, ParseError> {
        let mut lastCharacterIndex = self.nextCharacterIndex;
        while let Some(character) = self.getNextChar() {
            match character {
                _ if isWhitespaceChar(character) => {
                    self.addBasicTokenExcludeLastChar(true)?;
                }
                _ if isIdentifierCharacter(character)
                    || isNumberChar(character) => {
                    fn setExclusiveOptions(basicTokenOption: &mut BasicTokenOption, operator: bool, word: bool, number: bool) {
                        basicTokenOption.setPotentialExclusiveOptions(&[
                            if operator { Some(Operator) } else { None },
                            if word { Some(Word) } else { None },
                            if number { Some(Number) } else { None },
                        ]);
                    }

                    let operator = isTokenOperatorChar(character);
                    let outerNumber = isOuterNumberChar(character);
                    let number = isInnerNumberChar(character) || outerNumber;
                    let outerIdentifier = isOuterIdentifierCharacter(character);

                    if (operator && !self.basicTokenOption.isOptionSet(Operator)) || (!operator && self.basicTokenOption.isOptionSet(Operator) && self.lastTokenStart < lastCharacterIndex) {
                        self.addBasicTokenExcludeLastChar(false)?;
                    }

                    if self.isFirstCharacterInBasicToken() {
                        setExclusiveOptions(&mut self.basicTokenOption, operator, outerIdentifier, outerNumber)
                    } else {
                        setExclusiveOptions(&mut self.basicTokenOption, operator, isInnerIdentifierCharacter(character) || outerIdentifier, number);
                    }
                }
                ',' => {
                    self.addBasicTokenExcludeLastChar(true)?;
                    self.parserVec.addComma();
                }
                '/' if self.peekNextChar('/') => {
                    self.addBasicTokenExcludeLastChar(false)?;
                    // line comment
                    let commentStart = self.basicTokenStart;
                    while let Some(character) = self.getNextChar() {
                        if isEndOfLine(character) {
                            break;
                        }
                    }
                    self.addToken(TokenType::Comment(self.getFileRange(commentStart..self.nextCharacterIndex)));
                }
                '/' if self.peekNextChar('*') => {
                    self.addBasicTokenExcludeLastChar(false)?;
                    // multi-line comment
                    let commentStart = self.basicTokenStart;
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
                            return Err(self.getErrorBasicTokenRange(format!("reached end of file before finding closing */")));
                        }
                    }
                    debug_assert!(self.nextCharacterIndex >= 4); // enough space for /**/
                    self.addToken(TokenType::Comment(self.getFileRange(commentStart..self.nextCharacterIndex - 2)));
                }
                _ if isTokenOperatorChar(character) => {
                    if self.basicTokenOption.isExactlyOneOf(&[Word, Number]) {
                        self.addBasicTokenExcludeLastChar(false)?;
                    }
                    self.basicTokenOption.setExclusiveOptions(&[
                        Operator
                    ]);
                }
                // if let guards are currently unstable, call isQuoteChar/isOpenParenthesis/isCloseParenthesis twice with same parameters
                _ if isQuoteChar(character).is_some() => {
                    self.addBasicTokenExcludeLastChar(false)?;
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
                            return Err(self.getErrorBasicTokenRange(format!("missing closing {} quote", closeQuote)));
                        }
                    }

                    self.addToken(TokenType::String(quoteType, self.getFileRange(startIndex..self.nextCharacterIndex)));
                }
                _ if isOpenParenthesis(character).is_some() => {
                    self.addBasicTokenExcludeLastChar(true)?;
                    self.parenthesisSet.push(lastCharacterIndex, &mut self.parserVec, isOpenParenthesis(character).unwrap());
                }
                c if isCloseParenthesis(character).is_some() => {
                    self.addBasicTokenExcludeLastChar(true)?;
                    let parenthesisType = isCloseParenthesis(character).unwrap();
                    match self.parenthesisSet.pop(parenthesisType) {
                        Ok((parentTokenVec, openIndex)) => {
                            let tokenVec = self.parserVec.makeParent(self.nextCharacterIndex, parentTokenVec);
                            self.parserVec.addTokenRange(TokenType::Parenthesis(parenthesisType, tokenVec), self.getFileRange(openIndex..self.nextCharacterIndex));
                        }
                        Err(err) => {
                            return Err(self.getErrorBasicTokenRange(if let Some(parenthesis) = err {
                                format!("found closing '{c}' parenthesis, expected closing {}", parenthesis.parenthesisType.getClosing())
                            } else {
                                format!("unmatched closing '{c}' parenthesis")
                            }));
                        }
                    }
                }
                ';' => {
                    self.addBasicTokenExcludeLastChar(false)?;
                    self.parserVec.foldCommaVec(self.nextCharacterIndex - 1);
                    self.addToken(TokenType::SemiColan);
                }
                _ => return Err(self.getErrorBasicTokenRange(format!("unexpected character {character}")))
            }
            lastCharacterIndex = self.nextCharacterIndex;
        }
        self.addCurrentBasicToken()?;

        if let Some(parenthesis) = self.parenthesisSet.parenthesisVec.last() {
            let index = parenthesis.openingIndex;
            return Err(self.getErrorRange(index..self.nextCharacterIndex, self.parenthesisSet.parenthesisVec.iter().map(
                |parenthesis| format!("unmatched opening {} parenthesis", parenthesis.parenthesisType.getOpening())
            ).collect::<Vec<String>>().join(", ")));
        }

        return Ok(self.parserVec.takeTokenVec(self.nextCharacterIndex));
    }
}
