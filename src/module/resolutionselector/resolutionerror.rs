use crate::module::resolutionselector::resolutionconstraint::ResolutionConstraint;
use crate::module::typeinfo::Type;

pub enum ResolutionError {
    // forced to be both types
    Conflict(Vec<Type>),
    // forced type with invalid constraint
    ConstraintFailure(Vec<ResolutionConstraint>),
    // could be one of several possibilities
    Ambiguous(Vec<Type>),
    // all possibilities eliminated
    Eliminated,
    // not enough information constraining possible types
    //  (similar to ambiguous, but where the set of possibilities is indeterminate or infinite)
    Unconstrained,
}
