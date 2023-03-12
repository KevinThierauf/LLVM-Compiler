use crate::resolver::typeinfo::TypeInfo;

pub mod integer;
pub mod float;
pub mod character;
pub mod boolean;

pub trait PrimitiveTypeInfo: TypeInfo {}
