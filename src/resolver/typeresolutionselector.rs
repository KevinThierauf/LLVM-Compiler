use crate::ast::SymbolPos;
use crate::resolver::typeresolutionselector::resolutionconstraint::{ResolutionConstraint, ResolutionConstraintType};
use crate::resolver::typeresolutionselector::resolutionconstraintsolver::ResolutionConstraintSolver;
use crate::resolver::typeresolutionselector::typeresolutionerror::TypeResolutionError;
use crate::resolver::typeinfo::Type;

pub mod resolutionconstraint;
pub mod resolutionconstraintsolver;
pub mod typeresolutionerror;

enum ResolutionState {
    Valid(Type),
    Invalid(Vec<TypeResolutionError>),
    Unresolved,
}

pub struct TypeResolutionSelector {
    resolutionState: ResolutionState,
    constraints: Vec<ResolutionConstraint>,
}

impl TypeResolutionSelector {
    pub fn new() -> Self {
        return Self {
            resolutionState: ResolutionState::Unresolved,
            constraints: Vec::new(),
        };
    }

    pub fn newFrom(pos: SymbolPos, constraint: ResolutionConstraintType) -> Self {
        let mut v = Self::new();
        v.setTypeResolutionConstraint(pos, constraint);
        return v;
    }

    pub fn isResolved(&self) -> bool {
        return !matches!(&self.resolutionState, ResolutionState::Unresolved);
    }

    pub fn setTypeResolutionConstraint(&mut self, pos: SymbolPos, constraint: ResolutionConstraintType) {
        self.constraints.push(ResolutionConstraint::new(pos, constraint));
    }

    pub fn getResolvedExprType(&self) -> Option<Result<Type, &Vec<TypeResolutionError>>> {
        return match &self.resolutionState {
            ResolutionState::Valid(typeInfo) => Some(Ok(typeInfo.to_owned())),
            ResolutionState::Invalid(errorVec) => Some(Err(errorVec)),
            ResolutionState::Unresolved => None,
        };
    }

    pub fn resolve(&mut self) {
        let mut selector = ResolutionConstraintSolver::new();

        for constraint in &self.constraints {
            constraint.resolve(&mut selector);
        }

        self.resolutionState = match selector.take() {
            Ok(typeInfo) => ResolutionState::Valid(typeInfo),
            Err(errorVec) => ResolutionState::Invalid(errorVec)
        }
    }
}

