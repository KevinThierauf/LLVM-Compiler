use std::mem::swap;

use hashbrown::HashSet;

use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::module::typeinfo::{Type, TypeInfo};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ResolutionError {
    // forced to be both types
    Conflict(Type, Type),
    // could be one of several possibilities
    Ambiguous(HashSet<Type>),
    // all possibilities eliminated
    Eliminated,
    //
    Unconstrained,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ResolutionConstraintType {
    Exact(Type),
    Implicit(Type),
    Explicit(Type),
}

impl ResolutionConstraintType {
    pub fn isAllowed(&self, typeInfo: Type) -> bool {
        return match self {
            ResolutionConstraintType::Exact(ty) => *ty == typeInfo,
            ResolutionConstraintType::Implicit(ty) => todo!(),
            ResolutionConstraintType::Explicit(ty) => todo!(),
        };
    }
}

pub struct ResolutionConstraint {
    moduleRange: ModuleRange,
    constraintType: ResolutionConstraintType,
}

enum ResolutionState {
    Valid(Type),
    Invalid(Vec<ResolutionError>),
    Unresolved,
}

pub struct ResolutionConstraintSolver {
    resolutionState: ResolutionState,
    constraints: Vec<ResolutionConstraint>,
}

impl ResolutionConstraintSolver {
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

    fn setTypeResolutionConstraint(&mut self, moduleRange: ModuleRange, constraint: ResolutionConstraintType) {
        self.constraints.push(ResolutionConstraint {
            moduleRange,
            constraintType: constraint
        });
    }

    fn getResolvedExprType(&self) -> Option<Result<Type, &Vec<ResolutionError>>> {
        return match &self.resolutionState {
            ResolutionState::Valid(typeInfo) => Some(Ok(typeInfo.to_owned())),
            ResolutionState::Invalid(errorVec) => Err(errorVec),
            ResolutionState::Unresolved => None,
        }
    }

    pub fn resolve(&mut self) {
        todo!()
    }
}
