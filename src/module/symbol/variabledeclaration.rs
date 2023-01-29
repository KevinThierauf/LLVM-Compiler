use crate::module::modulepos::ModuleRange;
use crate::module::typeinfo::Type;

pub struct VariableDeclaration {
    variableName: ModuleRange,
    explicitType: Option<Type>,
}