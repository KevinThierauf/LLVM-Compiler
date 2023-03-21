use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};

pub static BOOLEAN_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Boolean { explicitConversions: vec![] })));

pub struct Boolean {
    explicitConversions: Vec<Type>,
}

impl TypeInfo for Boolean {
    fn getTypeName(&self) -> &str {
        return "bool";
    }

    fn getStaticSize(&self) -> u32 {
        return 1;
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }
}
