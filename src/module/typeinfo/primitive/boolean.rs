use std::marker::PhantomData;
use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::module::typeinfo::{Type, TypeInfo};

pub static BOOLEAN_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Boolean { phantom: PhantomData })));

pub struct Boolean {
    // prevent construction
    phantom: PhantomData<()>,
}

impl TypeInfo for Boolean {
    fn getTypeName(&self) -> &str {
        return "bool";
    }

    fn getStaticSize(&self) -> u32 {
        return 1;
    }

    fn getImplicitConversions(&self) -> &Vec<Type> {
        todo!()
    }
}
