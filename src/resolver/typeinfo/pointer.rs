use std::ops::Deref;
use std::sync::Arc;

use llvm_sys::core::LLVMPointerTypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

use crate::resolver::resolvedast::defaultpointer::DefaultPointer;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::typeinfo::{Type, TypeInfo};

pub struct PointerType {
    typeName: String,
}

impl PointerType {
    pub fn new(base: Type) -> Type {
        return Type(Arc::new(Self {
            typeName: format!("{}*", base.getTypeName()),
        }));
    }
}

impl TypeInfo for PointerType {
    fn getTypeName(&self) -> &str {
        return &self.typeName;
    }

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        return unsafe {
            LLVMPointerTypeInContext(context, 0)
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        static EMPTY_VEC: Lazy<Vec<Type>> = Lazy::new(|| Vec::new());
        return EMPTY_VEC.deref();
    }

    fn getDefaultValue(&self, ty: Type) -> ResolvedExpr {
        return ResolvedExpr::DefaultPointer(DefaultPointer {
            ty
        });
    }
}