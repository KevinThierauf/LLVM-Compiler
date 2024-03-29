use std::sync::Arc;

use llvm_sys::core::LLVMInt32TypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr::LiteralInteger;
use crate::resolver::typeinfo::{Type, TypeInfo};
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;

pub static INTEGER_TYPE: Lazy<Type> = Lazy::new(|| Integer::new("int"));

pub struct Integer {
    typeName: String,
}

impl Integer {
    pub fn new(typeName: impl Into<String>) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
        }));
    }
}

impl TypeInfo for Integer {
    fn getTypeName(&self) -> &str {
        return &self.typeName;
    }

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        return unsafe {
            LLVMInt32TypeInContext(context)
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        static EXPLICIT_CONVERSIONS: Lazy<Vec<Type>> = Lazy::new(|| vec![FLOAT_TYPE.to_owned()]);
        return &EXPLICIT_CONVERSIONS;
    }

    fn getDefaultValue(&self, _ty: Type) -> ResolvedExpr {
        return LiteralInteger(0);
    }

    fn isArithmeticType(&self) -> bool {
        return true;
    }
}
