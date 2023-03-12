use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

pub struct FunctionImpl {
}

impl FunctionImpl {
    pub fn getFunctionName(&self) -> &str {
        todo!()
    }
}

#[derive(Clone)]
pub struct Function(pub Arc<FunctionImpl>);

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}()", self.0.getFunctionName());
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
