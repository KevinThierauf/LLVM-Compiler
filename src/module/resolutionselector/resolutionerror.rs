use crate::module::modulepos::ModuleRange;
use crate::module::typeinfo::Type;

pub enum ResolutionError {
    // forced to be multiple types
    Conflict(Vec<(Type, Vec<ModuleRange>)>),
    // forced type with invalid subset constraint
    ForcedConstraint(Type, Vec<ModuleRange>),
    // selected type explicitly excluded
    Excluded(Vec<ModuleRange>),
    // could be one of several possibilities
    Ambiguous(Vec<Type>),
    // all possibilities eliminated
    Eliminated,
}
