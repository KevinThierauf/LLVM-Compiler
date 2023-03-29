use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use crate::ast::visibility::Visibility;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct Parameter {
    pub ty: Type,
    pub name: String,
}

pub struct FunctionImpl {
    pub name: String,
    pub returnType: Type,
    pub visibility: Visibility,
    pub parameters: Vec<Parameter>,
    pub id: usize,
}

impl FunctionImpl {
    pub fn getFunctionName(&self) -> &String {
        return &self.name;
    }
}

#[derive(Clone)]
pub struct Function(Arc<FunctionImpl>);

impl Function {
    pub fn new(name: String, visibility: Visibility, returnType: Type, parameters: Vec<Parameter>) -> Self {
        static NEXT_FUNCTION_ID: AtomicUsize = AtomicUsize::new(0);

        return Self {
            0: Arc::new(FunctionImpl {
                name,
                returnType,
                visibility,
                parameters,
                id: NEXT_FUNCTION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            }),
        };
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?} {} {}({})", self.0.visibility, self.returnType.getTypeName(), self.0.getFunctionName(), self.parameters.iter().map(|parameter| format!("{} {}", parameter.ty.getTypeName(), parameter.name)).collect::<Vec<_>>().join(", "));
    }
}

impl Deref for Function {
    type Target = Arc<FunctionImpl>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialOrd<Self> for Function {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0));
    }
}

impl Ord for Function {
    fn cmp(&self, other: &Self) -> Ordering {
        return Arc::as_ptr(&self.0).cmp(&Arc::as_ptr(&other.0));
    }
}

impl PartialEq<Self> for Function {
    fn eq(&self, other: &Self) -> bool {
        return Arc::ptr_eq(&self.0, &other.0);
    }
}

impl Eq for Function {}
