use crate::module::modulepos::ModulePos;
use crate::resolver::function::Function;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub enum ResolutionError {
    Unsupported(ModulePos, String),
    Unexpected(ModulePos, String),
    CircularDependencies(Vec<String>),
    // class field declared with let, but no default expr provided
    ResolutionClassField(ModulePos),
    // operation cannot be applied to value
    InvalidOperation(String),
    // operation cannot be applied to type
    InvalidOperationType(Type, String),
    ExpectedType(Type, Type, String),
    UnknownType(String),
    UnknownFunction(String),
    UnresolvedType(ModulePos, String),
    ParameterMismatch(Function, String),
    // conflicting field name (type name, field name)
    ConflictingFields(String, String),
    ConflictingType(Type, Type),
    ConflictingFunction(Function, Function),
    // function name
    ConflictingParameterName(String),
    // type name
    ConflictingTypeDefinition(String),
}
