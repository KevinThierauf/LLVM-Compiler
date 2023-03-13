use once_cell::sync::Lazy;

use crate::resolver::typeinfo::Type;
use crate::resolver::typeinfo::class::ClassTypeInfo;

pub static STRING_TYPE: Lazy<Type> = Lazy::new(|| {
    let mut classType = ClassTypeInfo::newBuilder("String");
    
    // todo
    
    classType.build()
});
