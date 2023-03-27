use std::sync::Arc;

use llvm_sys::core::LLVMFloatType;
use llvm_sys::prelude::LLVMTypeRef;
use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;

pub static FLOAT_TYPE: Lazy<Type> = Lazy::new(|| Float::new("float", 32));

pub struct Float {
    typeName: String,
    bitWidth: u32,
}

impl Float {
    pub fn new(typeName: impl Into<String>, bitWidth: u32) -> Type {
        return Type(Arc::new(Self {
            typeName: typeName.into(),
            bitWidth,
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

    fn getLLVMType(&self) -> LLVMTypeRef {
        return unsafe {
            LLVMFloatType()
        };
    }

    fn getExplicitConversions(&self) -> &Vec<Type> {
        static EXPLICIT_CONVERSIONS: Lazy<Vec<Type>> = Lazy::new(|| vec![INTEGER_TYPE.to_owned()]);
        return &EXPLICIT_CONVERSIONS;
    }

    fn isArithmeticType(&self) -> bool {
        return true;
    }
}
