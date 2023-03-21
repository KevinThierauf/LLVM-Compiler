use std::sync::Arc;

use crate::ast::visibility::Visibility;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::typeinfo::{Type, TypeInfo};

pub struct ClassField {
    pub visibility: Visibility,
    pub ty: Type,
    pub name: String,
}

pub struct ClassTypeInfo {
    name: String,
    staticSize: u32,
    visibility: Visibility,
    fieldVec: Vec<ClassField>,
    explicitConversions: Vec<Type>,
}

impl ClassTypeInfo {
    pub fn newBuilder(name: impl Into<String>, visibility: Visibility) -> Self {
        return Self {
            name: name.into(),
            staticSize: 0,
            visibility,
            fieldVec: Vec::new(),
            explicitConversions: Vec::new(),
        }
    }

    pub fn addField(&mut self, field: ClassField) -> Result<(), ResolutionError> {
        if self.fieldVec.iter().any(|f| f.name == field.name) {
            return Err(ResolutionError::ConflictingFields(self.name.to_owned(), field.name));
        }
        self.staticSize += field.ty.getStaticSize();
        self.fieldVec.push(field);
        return Ok(());
    }

    pub fn addFieldFrom(&mut self, visibility: Visibility, ty: Type, name: String) -> Result<(), ResolutionError> {
        return self.addField(ClassField {
            visibility,
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

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }
}
