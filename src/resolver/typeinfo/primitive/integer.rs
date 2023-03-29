use std::sync::Arc;

use llvm_sys::core::LLVMInt32TypeInContext;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;

pub static INTEGER_TYPE: Lazy<Type> = Lazy::new(|| Integer::new("int", 32));

pub struct Integer {
    typeName: String,
    bitWidth: u32,
}

impl Integer {
    pub fn new(typeName: impl Into<String>, bitWidth: u32) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
            bitWidth,
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

    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef {
        return unsafe {
            LLVMInt32TypeInContext(context)
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        static EXPLICIT_CONVERSIONS: Lazy<Vec<Type>> = Lazy::new(|| vec![FLOAT_TYPE.to_owned()]);
        return &EXPLICIT_CONVERSIONS;
    }

    fn isArithmeticType(&self) -> bool {
        return true;
    }
}
