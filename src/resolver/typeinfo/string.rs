use once_cell::sync::Lazy;

use crate::resolver::typeinfo::class::ClassTypeInfo;
use crate::resolver::typeinfo::pointer::PointerType;
use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::resolver::typeinfo::primitive::integer::{INTEGER_TYPE};
use crate::resolver::typeinfo::Type;

pub static STRING_TYPE: Lazy<Type> = Lazy::new(|| {
    let mut classType = ClassTypeInfo::newBuilder("String");
    classType.addFieldFrom(INTEGER_TYPE.to_owned(), "length".to_owned()).expect("failed to create string type");
    classType.addFieldFrom(PointerType::new(CHARACTER_TYPE.to_owned()), "pointer".to_owned()).expect("failed to create string type");

    classType.build()
});
