use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use hashbrown::HashSet;

pub mod tuple;
pub mod arraystatic;
pub mod void;
pub mod primitive;
pub mod class;
pub mod arraydynamic;

pub trait TypeInfo: Sync + Send {
    fn getTypeName(&self) -> &str;
    fn getStaticSize(&self) -> u32;
    fn getImplicitConversions(&self) -> &HashSet<Type>;
}

#[derive(Clone)]
pub struct Type(pub Arc<dyn TypeInfo>);

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

impl PartialEq<Self> for Type {
    fn eq(&self, other: &Self) -> bool {
        return Arc::ptr_eq(&self.0, &other.0);
    }
}

impl Eq for Type {
}
