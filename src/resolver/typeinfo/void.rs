use std::sync::Arc;

use llvm_sys::core::LLVMVoidTypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::typeinfo::{Type, TypeInfo};

pub static VOID_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Void { explicitConversions: vec![] })));

pub struct Void {
    explicitConversions: Vec<Type>,
}

impl TypeInfo for Void {
    fn getTypeName(&self) -> &str {
        return "void";
    }

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        return unsafe {
            LLVMVoidTypeInContext(context)
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        return &self.explicitConversions;
    }

    fn getDefaultValue(&self, _ty: Type) -> ResolvedExpr {
        unreachable!()
    }
}
