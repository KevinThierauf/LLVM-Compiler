use crate::module::typeinfo::TypeInfo;

pub struct ArrayDynamic<T: TypeInfo> {
    baseType: T
}
