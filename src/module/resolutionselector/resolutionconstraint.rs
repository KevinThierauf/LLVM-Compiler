use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::resolutionconstraintsolver::ResolutionConstraintSolver;
use crate::module::typeinfo::Type;

pub const DEFAULT_RESOLUTION_PRIORITY: u16 = 0;
pub const NO_IMPLICIT_CONVERSION: u16 = DEFAULT_RESOLUTION_PRIORITY + 100;

pub struct ResolutionConstraint {
    moduleRange: ModuleRange,
    constraintType: ResolutionConstraintType,
}

impl ResolutionConstraint {
    pub fn new(moduleRange: ModuleRange, constraintType: ResolutionConstraintType) -> Self {
        return Self {
            moduleRange,
            constraintType,
        };
    }

    pub fn resolve(&self, selector: &mut ResolutionConstraintSolver) {
        self.constraintType.resolve(&self.moduleRange, selector);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ResolutionConstraintType {
    Exact(Type),
    Implicit(Type),
    Or(Vec<ResolutionConstraintType>),
}

impl ResolutionConstraintType {
    pub fn resolve(&self, range: &ModuleRange, selector: &mut ResolutionConstraintSolver) {
        match self {
            ResolutionConstraintType::Exact(typeInfo) => {
                // forced type will always have priority
                selector.setForced(typeInfo);
            }
            ResolutionConstraintType::Implicit(typeInfo) => {
                selector.setPriority(typeInfo, NO_IMPLICIT_CONVERSION);
                selector.setSubset(typeInfo.getImplicitConversions());
            }
            ResolutionConstraintType::Or(options) => {
                selector.setAnyOf(range, options.as_slice());
            }
        }
    }
}
