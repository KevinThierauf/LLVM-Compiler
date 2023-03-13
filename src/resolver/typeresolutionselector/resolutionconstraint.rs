use crate::ast::SymbolPos;
use crate::resolver::typeresolutionselector::resolutionconstraintsolver::ResolutionConstraintSolver;
use crate::resolver::typeinfo::Type;

pub const DEFAULT_RESOLUTION_PRIORITY: u16 = 0;
pub const NO_IMPLICIT_CONVERSION: u16 = DEFAULT_RESOLUTION_PRIORITY + 100;

pub struct ResolutionConstraint {
    SymbolPos: SymbolPos,
    constraintType: ResolutionConstraintType,
}

impl ResolutionConstraint {
    pub fn new(SymbolPos: SymbolPos, constraintType: ResolutionConstraintType) -> Self {
        return Self {
            SymbolPos,
            constraintType,
        };
    }

    pub fn resolve(&self, selector: &mut ResolutionConstraintSolver) {
        self.constraintType.resolve(&self.SymbolPos, selector);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ResolutionConstraintType {
    Exact(Type),
    Implicit(Type),
}

impl ResolutionConstraintType {
    pub fn resolve(&self, pos: &SymbolPos, selector: &mut ResolutionConstraintSolver) {
        match self {
            ResolutionConstraintType::Exact(typeInfo) => {
                // forced type will always have priority
                selector.setForced(typeInfo, pos.to_owned());
            }
            ResolutionConstraintType::Implicit(typeInfo) => {
                selector.setPriority(typeInfo, NO_IMPLICIT_CONVERSION);
                selector.setSubsetOrdered(typeInfo.getImplicitConversions(), pos.to_owned());
            }
        }
    }
}
