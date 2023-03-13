use crate::ast::SymbolPos;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
#[cfg_attr(debug_assertions, derive(Eq, PartialEq, Ord, PartialOrd))]
pub enum TypeResolutionError {
    // forced to be multiple types
    Conflict(Vec<(Type, Vec<SymbolPos>)>),
    // forced type explicitly excluded
    ForcedExcluded { forced: Type, forcedRange: Vec<SymbolPos>, excludedRange: Vec<SymbolPos> },
    // forced type with invalid subset constraint
    ForcedSubset { forced: Type, forcedRange: Vec<SymbolPos>, excludedRange: Vec<SymbolPos> },
    // selected type explicitly excluded
    Excluded { selected: Type, excludedRange: Vec<SymbolPos> },
    // could be one of several possibilities
    Ambiguous(Vec<Type>),
    // all possibilities eliminated
    Eliminated,
    // not enough information provided
    Unconstrained,
}
