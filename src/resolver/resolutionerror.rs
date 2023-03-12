use crate::resolver::function::Function;
use crate::resolver::resolutionselector::typeresolutionerror::TypeResolutionError;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub enum ResolutionError {
    TypeResolutionError(TypeResolutionError),
    ConflictingTypeDefinition(Type, Type),
    ConflictingFunctionDefinition(Function, Function),
}
