use crate::module::typeinfo::TypeInfo;

pub struct Array<T: TypeInfo> {
    baseType: T,
    length: usize,
}
