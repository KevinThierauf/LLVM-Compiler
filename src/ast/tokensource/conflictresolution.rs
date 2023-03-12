use std::fmt::Debug;
use std::mem::swap;

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolDiscriminants};
use crate::ast::symbol::classdefinition::{ClassMember, ClassMemberDiscriminants};
use crate::ast::tokensource::symbolparser::Match;
use crate::module::modulepos::ModulePos;

struct ConflictResolver<'a, S: Debug, SD: Copy + Eq + for<'b> From<&'b S>> {
    pos: ModulePos,
    indexVec: Vec<usize>,
    options: Vec<&'a Match<S>>,
    preferVec: Vec<(SD, SD, bool)>,
}

impl<'a, S: Debug, SD: Copy + Eq + for<'b> From<&'b S>> ConflictResolver<'a, S, SD> {
    fn new(pos: ModulePos, options: Vec<&'a Match<S>>) -> Self {
        let mut indexVec = Vec::new();
        for index in 0..options.len() {
            indexVec.push(index);
        }
        return Self {
            pos,
            indexVec,
            options,
            preferVec: Vec::new(),
        };
    }

    fn removeIndex(&mut self, index: usize) {
        self.indexVec.swap_remove(index);
    }

    fn removeIf(&mut self, mut f: impl FnMut(&'a Match<S>) -> bool) {
        let mut indexVecIndex = 0;
        while indexVecIndex < self.indexVec.len() {
            if f(self.options[self.indexVec[indexVecIndex]]) {
                self.removeIndex(indexVecIndex);
            } else {
                indexVecIndex += 1;
            }
        }
    }

    fn removeSymbol(&mut self, value: SD) {
        self.removeIf(move |symbol| SD::from(symbol.getValue()) == value);
    }

    fn contains(&self, value: SD) -> Option<(&Match<S>, usize)> {
        for index in &self.indexVec {
            if SD::from(self.options[*index].getValue()) == value {
                return Some((self.options[*index], *index));
            }
        }
        return None;
    }

    fn setPreferred(&mut self, prefer: SD, over: SD) {
        self.preferVec.push((prefer, over, false));
    }

    fn setPreferredOnlyIfLonger(&mut self, prefer: SD, over: SD) {
        self.preferVec.push((prefer, over, true));
    }

    fn getResolved(mut self) -> Result<usize, ASTError> {
        if self.options.is_empty() {
            return Err(ASTError::MatchFailed(self.pos));
        }

        let mut preferVec = Vec::new();
        swap(&mut self.preferVec, &mut preferVec);
        for (preferredDiscriminant, over, longerOnly) in preferVec {
            if let Some((preferred, _)) = self.contains(preferredDiscriminant) {
                if longerOnly {
                    if let Some((over, overIndex)) = self.contains(over) {
                        let preferredLength = preferred.getRange().getLength();
                        let overLength = over.getRange().getLength();

                        if overLength <= preferredLength {
                            self.removeIndex(overIndex);
                        } else {
                            self.removeSymbol(preferredDiscriminant);
                        }
                    }
                } else {
                    self.removeSymbol(over);
                }
            }
        }

        return match self.indexVec.as_slice() {
            [] => Err(ASTError::EliminatedConflict(self.pos, self.options.into_iter().map(|symbol| format!("{:?}", symbol.getValue())).collect())),
            [index] => Ok(*index),
            _ => Err(ASTError::MultipleConflict(self.pos, self.indexVec.into_iter().map(|index| (
                format!("{}", self.options[index].getRange().getSource()),
                self.options[index].getRange().getTokens().iter().map(|token| format!("{:?}", token)).collect(),
                format!("{:?}", self.options[index])
            )).collect()))
        };
    }
}

pub fn resolveClassDefinitionConflict<'a>(pos: ModulePos, options: impl Iterator<Item = &'a Match<ClassMember>>) -> Result<usize, ASTError> {
    let mut resolver = ConflictResolver::new(pos, options.collect());
    
    resolver.setPreferred(ClassMemberDiscriminants::FunctionDefinition, ClassMemberDiscriminants::FieldDefinition);
    
    return resolver.getResolved();
}

pub fn resolveSymbolConflict<'a>(pos: ModulePos, options: impl Iterator<Item = &'a Match<Symbol>>) -> Result<usize, ASTError> {
    let mut resolver = ConflictResolver::new(pos, options.collect());

    resolver.setPreferred(SymbolDiscriminants::FunctionCall, SymbolDiscriminants::Variable);
    resolver.setPreferred(SymbolDiscriminants::VariableDeclaration, SymbolDiscriminants::Variable);
    resolver.setPreferred(SymbolDiscriminants::FunctionDefinition, SymbolDiscriminants::VariableDeclaration);
    resolver.setPreferred(SymbolDiscriminants::FunctionDefinition, SymbolDiscriminants::LiteralVoid);

    resolver.setPreferred(SymbolDiscriminants::While, SymbolDiscriminants::Operator);
    resolver.setPreferred(SymbolDiscriminants::While, SymbolDiscriminants::Variable);

    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::FunctionCall);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::VariableDeclaration);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::Variable);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralArray);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralBool);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralChar);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralFloat);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralInteger);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralString);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralVoid);
    resolver.setPreferredOnlyIfLonger(SymbolDiscriminants::Operator, SymbolDiscriminants::LiteralTuple);

    return resolver.getResolved();
}
