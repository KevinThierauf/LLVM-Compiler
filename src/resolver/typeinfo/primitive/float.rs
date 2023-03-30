use std::sync::Arc;

use llvm_sys::core::LLVMFloatTypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr::LiteralFloat;
use crate::resolver::typeinfo::{Type, TypeInfo};
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;

pub static FLOAT_TYPE: Lazy<Type> = Lazy::new(|| Float::new("float"));

pub struct Float {
    typeName: String,
}

impl Float {
    pub fn new(typeName: impl Into<String>) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
        }));
    }
}

impl TypeInfo for Float {
    fn getTypeName(&self) -> &str {
        return &self.typeName;
    }

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        return unsafe {
            LLVMFloatTypeInContext(context)
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        static EXPLICIT_CONVERSIONS: Lazy<Vec<Type>> = Lazy::new(|| vec![INTEGER_TYPE.to_owned()]);
        return &EXPLICIT_CONVERSIONS;
    }

    fn getDefaultValue(&self, _ty: Type) -> ResolvedExpr {
        return LiteralFloat(0.0);
    }

    fn isArithmeticType(&self) -> bool {
        return true;
    }
}
