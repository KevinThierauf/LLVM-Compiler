use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

use hashbrown::HashMap;
use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};
use once_cell::sync::Lazy;

pub mod tuple;
pub mod void;
pub mod primitive;
pub mod class;
pub mod string;

#[derive(Debug, Clone)]
pub struct TypeProperty {
    pub ty: Type,
    pub name: String,
}

pub trait TypeInfo: Sync + Send {
    fn getTypeName(&self) -> &str;
    fn getStaticSize(&self) -> u32;
    fn getLLVMType(&self, context: LLVMContextRef) -> LLVMTypeRef;
    fn getExplicitConversions(&self) -> &Vec<Type>;

    fn getPropertyMap(&self) -> &HashMap<String, TypeProperty> {
        static EMPTY_MAP: Lazy<HashMap<String, TypeProperty>> = Lazy::new(|| HashMap::new());
        return EMPTY_MAP.deref();
    }

    fn isArithmeticType(&self) -> bool {
        return false;
    }
}

#[derive(Clone)]
pub struct Type(pub Arc<dyn TypeInfo>);

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0.getTypeName());
    }
}

impl Deref for Type {
    type Target = Arc<dyn TypeInfo>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialOrd<Self> for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0));
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        return Arc::as_ptr(&self.0).cmp(&Arc::as_ptr(&other.0));
    }
}

impl PartialEq<Self> for Type {
    fn eq(&self, other: &Self) -> bool {
        return Arc::ptr_eq(&self.0, &other.0);
    }
}

impl Eq for Type {}

impl PartialEq<Lazy<Self>> for Type {
    fn eq(&self, other: &Lazy<Self>) -> bool {
        return self == other.deref();
    }
}
