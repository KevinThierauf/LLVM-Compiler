use std::mem::swap;

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolDiscriminants};
use crate::module::modulepos::ModulePos;

struct ConflictResolver<'a> {
    pos: ModulePos,
    indexVec: Vec<usize>,
    options: Vec<&'a Symbol>,
    preferVec: Vec<(SymbolDiscriminants, SymbolDiscriminants, bool)>,
}

impl<'a> ConflictResolver<'a> {
    fn new(pos: ModulePos, options: Vec<&'a Symbol>) -> Self {
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

    fn removeIf(&mut self, mut f: impl FnMut(&'a Symbol) -> bool) {
        let mut indexVecIndex = 0;
        while indexVecIndex < self.indexVec.len() {
            if f(self.options[self.indexVec[indexVecIndex]]) {
                self.removeIndex(indexVecIndex);
            } else {
                indexVecIndex += 1;
            }
        }
    }

    fn removeSymbol(&mut self, value: SymbolDiscriminants) {
        self.removeIf(move |symbol| SymbolDiscriminants::from(symbol) == value);
    }

    fn contains(&self, value: SymbolDiscriminants) -> Option<(&Symbol, usize)> {
        for index in &self.indexVec {
            if SymbolDiscriminants::from(self.options[*index]) == value {
                return Some((self.options[*index], *index));
            }
        }
        return None;
    }

    fn setPreferred(&mut self, prefer: SymbolDiscriminants, over: SymbolDiscriminants) {
        self.preferVec.push((prefer, over, false));
    }

    fn setPreferredOnlyIfLonger(&mut self, prefer: SymbolDiscriminants, over: SymbolDiscriminants) {
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
                        let preferredLength = preferred.getSymbolType().getRange().getLength();
                        let overLength = over.getSymbolType().getRange().getLength();

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
            [] => Err(ASTError::EliminatedConflict(self.pos, self.options.into_iter().map(|symbol| format!("{:?}", symbol)).collect())),
            [index] => Ok(*index),
            _ => Err(ASTError::MultipleConflict(self.pos, self.indexVec.into_iter().map(|index| (
                format!("{}", self.options[index].getSymbolType().getRange().getSource()),
                self.options[index].getSymbolType().getRange().getTokens().iter().map(|token| format!("{:?}", token)).collect(),
                format!("{:?}", self.options[index])
            )).collect()))
        };
    }
}

pub fn resolveConflict<'a>(pos: ModulePos, options: impl Iterator<Item=&'a Symbol>) -> Result<usize, ASTError> {
    let mut resolver = ConflictResolver::new(pos, options.collect());

    resolver.setPreferred(SymbolDiscriminants::FunctionCall, SymbolDiscriminants::Variable);
    resolver.setPreferred(SymbolDiscriminants::VariableDeclaration, SymbolDiscriminants::Variable);
    resolver.setPreferred(SymbolDiscriminants::FunctionDefinition, SymbolDiscriminants::VariableDeclaration);
    resolver.setPreferred(SymbolDiscriminants::FunctionDefinition, SymbolDiscriminants::LiteralVoid);

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
