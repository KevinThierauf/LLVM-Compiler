use crate::module::modulepos::ModuleRange;
use crate::resolver::resolutionselector::resolutionconstraint::{ResolutionConstraint, ResolutionConstraintType};
use crate::resolver::resolutionselector::resolutionconstraintsolver::ResolutionConstraintSolver;
use crate::resolver::resolutionselector::resolutionerror::ResolutionError;
use crate::ast::typeinfo::Type;

pub mod resolutionconstraint;
pub mod resolutionconstraintsolver;
pub mod resolutionerror;

enum ResolutionState {
    Valid(Type),
    Invalid(Vec<ResolutionError>),
    Unresolved,
}

pub struct ResolutionSelector {
    resolutionState: ResolutionState,
    constraints: Vec<ResolutionConstraint>,
}

impl ResolutionSelector {
    pub fn new() -> Self {
        return Self {
            resolutionState: ResolutionState::Unresolved,
            constraints: Vec::new(),
        };
    }

    pub fn newFrom(range: ModuleRange, constraint: ResolutionConstraintType) -> Self {
        let mut v = Self::new();
        v.setTypeResolutionConstraint(range, constraint);
        return v;
    }

    pub fn isResolved(&self) -> bool {
        return !matches!(&self.resolutionState, ResolutionState::Unresolved);
    }

    pub fn setTypeResolutionConstraint(&mut self, moduleRange: ModuleRange, constraint: ResolutionConstraintType) {
        self.constraints.push(ResolutionConstraint::new(moduleRange, constraint));
    }

    pub fn getResolvedExprType(&self) -> Option<Result<Type, &Vec<ResolutionError>>> {
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

