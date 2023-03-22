pub mod emit;

use crate::resolver::resolvedast::ResolvedAST;

pub struct CompiledModule {}

impl CompiledModule {
    pub fn new(resolved: ResolvedAST) -> Self {
        // todo
        return Self::empty();
    }

    pub fn empty() -> Self {
        return CompiledModule {};
    }

    pub fn merge(&mut self, other: CompiledModule) {
        // todo
    }
}
