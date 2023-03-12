use std::marker::PhantomData;
use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::resolver::typeinfo::{Type, TypeInfo};

pub static CHARACTER_TYPE: Lazy<Type> = Lazy::new(|| Type(Arc::new(Character { phantom: PhantomData })));

pub struct Character {
    // prevent construction
    phantom: PhantomData<()>,
}

impl TypeInfo for Character {
    fn getTypeName(&self) -> &str {
        return "char";
    }

    fn getStaticSize(&self) -> u32 {
        return 4;
    }

    fn getImplicitConversions(&self) -> &Vec<Type> {
        todo!()
    }
}
