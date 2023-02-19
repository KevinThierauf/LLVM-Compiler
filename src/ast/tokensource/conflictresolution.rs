use std::mem::swap;

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolDiscriminants};

struct ConflictResolver<'a> {
    indexVec: Vec<usize>,
    options: Vec<&'a Symbol>,
    preferVec: Vec<(SymbolDiscriminants, SymbolDiscriminants)>,
}

impl<'a> ConflictResolver<'a> {
    fn new(options: Vec<&'a Symbol>) -> Self {
        let mut indexVec = Vec::new();
        for index in 0..options.len() {
            indexVec.push(index);
        }
        return Self {
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

    fn contains(&self, value: SymbolDiscriminants) -> bool {
        for index in &self.indexVec {
            if SymbolDiscriminants::from(self.options[*index]) == value {
                return true;
            }
        }
        return false;
    }

    fn setPreferred(&mut self, prefer: SymbolDiscriminants, over: SymbolDiscriminants) {
        self.preferVec.push((prefer, over));
    }

    fn getResolved(mut self) -> Result<usize, ASTError> {
        let mut preferVec = Vec::new();
        swap(&mut self.preferVec, &mut preferVec);
        for (preferred, over) in preferVec {
            if self.contains(preferred) {
                self.removeSymbol(over);
            }
        }

        return match self.indexVec.as_slice() {
            [] => Err(ASTError::EliminatedConflict),
            [index] => Ok(*index),
            _ => Err(ASTError::MultipleConflict),
        };
    }
}

pub fn resolveConflict<'a>(options: impl Iterator<Item = &'a Symbol>) -> Result<usize, ASTError> {
    let mut resolver = ConflictResolver::new(options.collect());

    resolver.setPreferred(SymbolDiscriminants::FunctionCall, SymbolDiscriminants::Variable);

    return resolver.getResolved();
}
