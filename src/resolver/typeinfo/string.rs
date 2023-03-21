use once_cell::sync::Lazy;

use crate::ast::visibility::Visibility;
use crate::resolver::typeinfo::class::ClassTypeInfo;
use crate::resolver::typeinfo::Type;

pub static STRING_TYPE: Lazy<Type> = Lazy::new(|| {
    let mut classType = ClassTypeInfo::newBuilder("String", Visibility::Public);
    
    // todo

    classType.build()
});
