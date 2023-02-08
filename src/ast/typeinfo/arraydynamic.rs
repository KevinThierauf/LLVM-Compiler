use crate::ast::typeinfo::TypeInfo;

pub struct ArrayDynamic<T: TypeInfo> {
    baseType: T
}
