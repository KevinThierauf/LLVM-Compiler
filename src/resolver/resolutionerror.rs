use crate::ast::SymbolPos;
use crate::resolver::function::Function;
use crate::resolver::typeinfo::Type;
use crate::resolver::typeresolutionselector::typeresolutionerror::TypeResolutionError;

#[derive(Debug)]
pub enum ResolutionError {
    TypeResolutionError(TypeResolutionError),
    UnknownType(SymbolPos, String),
    UnknownFunction(SymbolPos, String),
    ConflictingType(SymbolPos, Type, Type),
    ConflictingFunction(SymbolPos, Function, Function),
}
