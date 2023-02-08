use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::ast::ASTError;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::conflictresolution::resolveConflict;
use crate::ast::tokensource::symbolresolver::{getRootNode, SymbolResolver};
use crate::module::{Token, TokenTypeDiscriminants};

pub struct TokenParser<'a> {
    tokenVector: &'a Vec<Token>,
}

impl<'a> TokenParser<'a> {
    pub fn new(tokenVector: &'a Vec<Token>) -> Self {
        return Self {
            tokenVector,
        };
    }

    fn getResolvedSymbol(mut symbolVec: Vec<(Symbol, usize)>) -> Result<(Symbol, usize), ASTError> {
        return match symbolVec.len() {
            0 => Err(ASTError::NoMatch),
            1 => Ok(symbolVec.remove(0)),
            _ => Ok(symbolVec.swap_remove(resolveConflict(symbolVec.iter().map(|(symbol, _)| symbol))?)),
        };
    }

    pub fn parse(self) -> Result<Vec<Symbol>, ASTError> {
        let mut symbolVec = Vec::new();
        let mut nodeParser = NodeParser::new(getRootNode());
        let mut tokenIndex = 0;
        let mut startSymbolIndex = 0;
        let tokenLength = self.tokenVector.len();
        while tokenIndex < tokenLength {
            let token = &self.tokenVector[tokenIndex];
            if nodeParser.addToken(token) {
                tokenIndex += 1;
            } else {
                let (symbol, index) = Self::getResolvedSymbol(nodeParser.getResolvedSymbolVec())?;
                symbolVec.push(symbol);
                assert_ne!(index, startSymbolIndex);
                tokenIndex = index;
                startSymbolIndex = index;
                nodeParser = NodeParser::new(getRootNode());
            }
        }

        if tokenIndex != startSymbolIndex {
            symbolVec.push(Self::getResolvedSymbol(nodeParser.getResolvedSymbolVec().into_iter().filter(|(_, index)| *index == startSymbolIndex).collect::<Vec<(Symbol, usize)>>())?.0);
        }

        return Ok(symbolVec);
    }
}

#[derive(Clone)]
pub struct Node {
    nodeMap: HashMap<TokenTypeDiscriminants, Node>,
    symbolResolver: Vec<SymbolResolver>,
}

impl Node {
    pub fn new() -> Self {
        return Self {
            nodeMap: Default::default(),
            symbolResolver: Vec::new(),
        };
    }

    pub fn newWith(tokens: &[TokenTypeDiscriminants]) -> Self {
        let mut value = Self::new();
        let mut nextNode = &mut value;
        for token in tokens {
            nextNode = nextNode.addNode(token.to_owned(), Node::new());
        }
        return value;
    }

    pub fn newWithResolved(tokens: &[TokenTypeDiscriminants], resolver: SymbolResolver) -> Self {
        let mut value = Self::newWith(tokens);
        value.addResolver(resolver);
        return value;
    }

    pub fn mergeNode(&mut self, mut node: Node) {
        self.symbolResolver.append(&mut node.symbolResolver);
        for (tokenType, node) in node.nodeMap {
            self.addNode(tokenType, node);
        }
    }

    pub fn addNode(&mut self, tokenType: TokenTypeDiscriminants, mut node: Node) -> &mut Node {
        return match self.nodeMap.entry(tokenType) {
            Entry::Occupied(occupied) => {
                let existingNode = occupied.into_mut();
                existingNode.symbolResolver.append(&mut node.symbolResolver);

                for (tokenType, nextNode) in node.nodeMap {
                    existingNode.addNode(tokenType, nextNode);
                }

                existingNode
            }
            Entry::Vacant(vacant) => {
                vacant.insert(node)
            }
        };
    }

    pub fn addResolver(&mut self, symbolResolver: SymbolResolver) {
        self.symbolResolver.push(symbolResolver);
    }
}

pub struct NodeBuilder<'a> {
    currentNode: &'a mut Node,
}

impl<'a> NodeBuilder<'a> {
    pub fn new(currentNode: &'a mut Node) -> Self {
        return Self {
            currentNode
        };
    }

    pub fn addToken(&mut self, token: TokenTypeDiscriminants) {
        self.currentNode = unsafe { &mut *(self.currentNode.addNode(token, Node::new()) as *mut _) };
    }

    pub fn mergeNode(&mut self, node: Node) {
        self.currentNode.mergeNode(node);
    }

    pub fn addResolver(&mut self, resolver: SymbolResolver) {
        self.currentNode.addResolver(resolver);
    }
}

struct NodeParser<'a> {
    tokenVec: Vec<&'a Token>,
    currentNode: Node,
    currentSymbolVec: Vec<(SymbolResolver, usize)>,
}

impl<'a> NodeParser<'a> {
    fn new(rootNode: Node) -> Self {
        return Self {
            tokenVec: vec![],
            currentNode: rootNode,
            currentSymbolVec: Vec::new(),
        };
    }

    fn addResolved(&mut self) {
        self.currentSymbolVec.extend(self.currentNode.symbolResolver.iter().map(|resolver| (resolver.to_owned(), self.tokenVec.len())));
    }

    fn addToken(&mut self, token: &'a Token) -> bool {
        self.addResolved();
        self.tokenVec.push(token);
        return if let Some(node) = self.currentNode.nodeMap.remove(&TokenTypeDiscriminants::from(token.getTokenType())) {
            self.currentNode = node;
            true
        } else {
            false
        };
    }

    fn getResolvedSymbolVec(mut self) -> Vec<(Symbol, usize)> {
        self.addResolved();
        return self.currentSymbolVec.into_iter().map(|(resolver, index)| (resolver.resolve(&self.tokenVec[0..index]), index)).collect();
    }
}
