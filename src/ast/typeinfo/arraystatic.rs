use crate::ast::typeinfo::TypeInfo;

pub struct Array<T: TypeInfo> {
    baseType: T,
    length: usize,
}
