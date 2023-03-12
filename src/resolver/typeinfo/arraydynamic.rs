use crate::resolver::typeinfo::TypeInfo;

pub struct ArrayDynamic<T: TypeInfo> {
    baseType: T,
}
