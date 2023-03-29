use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};

use crate::ast::visibility::Visibility;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::typeinfo::{Type, TypeInfo, TypeProperty};

#[derive(Debug)]
pub struct ClassField {
    pub visibility: Visibility,
    pub ty: Type,
    pub name: String,
}

#[derive(Debug)]
pub struct ClassTypeInfo {
    name: String,
    staticSize: u32,
    propertyMap: HashMap<String, TypeProperty>,
    explicitConversions: Vec<Type>,
}

impl ClassTypeInfo {
    pub fn newBuilder(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            staticSize: 0,
            propertyMap: HashMap::new(),
            explicitConversions: Vec::new(),
        };
    }

    pub fn addFieldFrom(&mut self, ty: Type, name: String) -> Result<(), ResolutionError> {
        return match self.propertyMap.entry(name.to_owned()) {
            Entry::Occupied(_) => {
                Err(ResolutionError::ConflictingFields(self.name.to_owned(), name))
            }
            Entry::Vacant(entry) => {
                self.staticSize += ty.getStaticSize();
                entry.insert(TypeProperty {
                    ty,
                    name,
                });
                Ok(())
            }
        };
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

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        todo!()
    }

    fn getDefaultValue(&self) -> ResolvedExpr {
        todo!()
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }

    fn getPropertyMap(&self) -> &HashMap<String, TypeProperty> {
        return &self.propertyMap;
    }
}
