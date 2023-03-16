use crate::module::modulepos::ModulePos;
use crate::resolver::function::Function;
use crate::resolver::typeinfo::Type;
use crate::resolver::typeresolutionselector::typeresolutionerror::TypeResolutionError;

#[derive(Debug)]
pub enum ResolutionError {
    Unsupported(ModulePos, String),
    CircularDependencies(Vec<String>),
    // class field declared with let, but no default expr provided
    ResolutionClassField(ModulePos),
    TypeResolutionError(TypeResolutionError),
    UnknownType(String),
    UnknownFunction(String),
    // conflicting field name (type name, field name)
    ConflictingFields(String, String),
    ConflictingType(Type, Type),
    ConflictingFunction(Function, Function),
    // function name
    ConflictingParameterName(String),
    // type name
    ConflictingTypeDefinition(String),
}
