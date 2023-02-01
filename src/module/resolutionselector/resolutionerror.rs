use crate::module::modulepos::ModuleRange;
use crate::module::typeinfo::Type;

#[derive(Debug)]
#[cfg_attr(debug_assertions, derive(Eq, PartialEq, Ord, PartialOrd))]
pub enum ResolutionError {
    // forced to be multiple types
    Conflict(Vec<(Type, Vec<ModuleRange>)>),
    // forced type with invalid subset constraint
    ForcedConstraint(Type, Vec<(Type, Vec<ModuleRange>)>),
    // selected type explicitly excluded
    Excluded(Type, Vec<ModuleRange>),
    // could be one of several possibilities
    Ambiguous(Vec<Type>),
    // all possibilities eliminated
    Eliminated,
    // not enough information provided
    Unconstrained,
}
