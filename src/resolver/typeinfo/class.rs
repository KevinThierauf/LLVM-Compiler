use std::sync::Arc;
use crate::resolver::function::Function;

use crate::resolver::typeinfo::{Type, TypeInfo};

pub struct ClassField {
    pub ty: Type,
    pub name: String,
}

pub struct ClassTypeInfo {
    name: String,
    staticSize: u32,
    fieldVec: Vec<ClassField>,
    functionVec: Vec<Function>,
    implicitConversions: Vec<Type>,
}

impl ClassTypeInfo {
    pub fn newBuilder(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            staticSize: 0,
            fieldVec: Vec::new(),
            functionVec: Vec::new(),
            implicitConversions: Vec::new(),
        }
    }

    pub fn addField(&mut self, ty: Type, name: String) {
        self.staticSize += ty.getStaticSize();
        self.fieldVec.push(ClassField {
            ty,
            name,
        });
    }

    pub fn build(self) -> Type {
        return Type(Arc::new(self));
    }
}

impl TypeInfo for ClassTypeInfo {
    fn getTypeName(&self) -> &str {
        return &self.name;
    }

    fn getStaticSize(&self) -> u32 {
        return self.staticSize;
    }

    fn getImplicitConversions(&self) -> &Vec<Type> {
        return &self.implicitConversions;
    }
}
