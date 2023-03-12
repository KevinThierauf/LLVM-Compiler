use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
#[cfg_attr(debug_assertions, derive(Eq, PartialEq, Ord, PartialOrd))]
pub enum ResolutionError {
    // forced to be multiple types
    Conflict(Vec<(Type, Vec<ModuleRange>)>),
    // forced type explicitly excluded
    ForcedExcluded { forced: Type, forcedRange: Vec<ModuleRange>, excludedRange: Vec<ModuleRange> },
    // forced type with invalid subset constraint
    ForcedSubset { forced: Type, forcedRange: Vec<ModuleRange>, excludedRange: Vec<ModuleRange> },
    // selected type explicitly excluded
    Excluded { selected: Type, excludedRange: Vec<ModuleRange> },
    // could be one of several possibilities
    Ambiguous(Vec<Type>),
    // all possibilities eliminated
    Eliminated,
    // not enough information provided
    Unconstrained,
}
