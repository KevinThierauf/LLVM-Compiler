use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::resolver::typeinfo::{Type, TypeInfo};

pub static FLOAT_TYPE: Lazy<Type> = Lazy::new(|| Float::new("float", 32, vec![]));

pub struct Float {
    typeName: String,
    bitWidth: u32,
    implicitConversions: Vec<Type>,
}

impl Float {
    pub fn new(typeName: impl Into<String>, bitWidth: u32, implicitConversions: Vec<Type>) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
            bitWidth,
            implicitConversions,
        }));
    }
}

impl TypeInfo for Float {
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
