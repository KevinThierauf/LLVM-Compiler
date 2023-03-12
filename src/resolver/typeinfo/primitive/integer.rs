use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::resolver::typeinfo::{Type, TypeInfo};

pub static INTEGER_TYPE: Lazy<Type> = Lazy::new(|| Integer::new("int", 32, vec![]));

pub struct Integer {
    typeName: String,
    bitWidth: u32,
    implicitConversions: Vec<Type>,
}

impl Integer {
    pub fn new(typeName: impl Into<String>, bitWidth: u32, implicitConversions: Vec<Type>) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
            bitWidth,
            implicitConversions,
        }));
    }
}

impl TypeInfo for Integer {
    fn getTypeName(&self) -> &str {
        return &self.typeName;
    }

    fn getStaticSize(&self) -> u32 {
        return self.bitWidth;
    }

    fn getImplicitConversions(&self) -> &Vec<Type> {
        return &self.implicitConversions;
    }
}
