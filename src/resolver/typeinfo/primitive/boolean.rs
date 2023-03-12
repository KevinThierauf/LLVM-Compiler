use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};

pub static BOOLEAN_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Boolean { implicitConversions: vec![] })));

pub struct Boolean {
    implicitConversions: Vec<Type>,
}

impl TypeInfo for Boolean {
    fn getTypeName(&self) -> &str {
        return "bool";
    }

    fn getStaticSize(&self) -> u32 {
        return 1;
    }

    fn getImplicitConversions(&self) -> &Vec<Type> {
        return &self.implicitConversions;
    }
}
