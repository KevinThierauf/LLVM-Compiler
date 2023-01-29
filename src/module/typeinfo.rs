use std::sync::Arc;

pub mod tuple;
pub mod arraystatic;
pub mod void;
pub mod primitive;
pub mod class;
pub mod arraydynamic;

pub trait TypeInfo: Sync + Send {
    fn getTypeName(&self) -> &str;
    fn getStaticSize(&self) -> u32;
}

#[derive(Clone)]
pub struct Type(pub Arc<dyn TypeInfo>);

impl PartialEq<Self> for Type {
    fn eq(&self, other: &Self) -> bool {
        return Arc::ptr_eq(&self.0, &other.0);
    }
}

impl Eq for Type {
}
