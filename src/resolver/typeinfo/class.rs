use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use llvm_sys::core::LLVMStructTypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use parking_lot::Mutex;

use crate::ast::visibility::Visibility;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::defaultclass::DefaultClass;
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
    llvmType: Mutex<Option<SendLLVMTypeRef>>,
}

#[derive(Debug)]
struct SendLLVMTypeRef(LLVMTypeRef);

unsafe impl Send for SendLLVMTypeRef {}

impl ClassTypeInfo {
    pub fn newBuilder(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            staticSize: 0,
            propertyMap: HashMap::new(),
            explicitConversions: Vec::new(),
            llvmType: Default::default(),
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
        return self.llvmType.lock().get_or_insert_with(|| {
            let mut llvmTypes = self.propertyMap.iter().map(|(_, property)| property.ty.getLLVMType(context)).collect::<Vec<_>>();
            unsafe {
                return SendLLVMTypeRef(LLVMStructTypeInContext(context, llvmTypes.as_mut_ptr(), llvmTypes.len() as _, 0 as _));
            }
        }).0;
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }

    fn getDefaultValue(&self, ty: Type) -> ResolvedExpr {
        return ResolvedExpr::DefaultClass(DefaultClass {
            ty,
        });
    }

    fn getPropertyMap(&self) -> &HashMap<String, TypeProperty> {
        return &self.propertyMap;
    }
}
