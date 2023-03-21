use once_cell::sync::Lazy;

use crate::resolver::typeinfo::class::ClassTypeInfo;
use crate::resolver::typeinfo::Type;

pub static STRING_TYPE: Lazy<Type> = Lazy::new(|| {
    let mut classType = ClassTypeInfo::newBuilder("String");

    // todo

    classType.build()
});
