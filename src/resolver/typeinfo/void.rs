use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};

pub static VOID_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Void { explicitConversions: vec![] })));

pub struct Void {
    explicitConversions: Vec<Type>,
}

impl TypeInfo for Void {
    fn getTypeName(&self) -> &str {
        return "void";
    }

    fn getStaticSize(&self) -> u32 {
        return 0;
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }
}