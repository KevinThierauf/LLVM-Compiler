use crate::resolver::resolvedast::ResolvedAST;

pub struct CompiledModule {
}

impl CompiledModule {
    pub fn new(resolved: ResolvedAST) -> Self {
        todo!()
    }
    
    pub fn empty() -> Self {
        return CompiledModule {
        };
    }
    
    pub fn merge(&mut self, other: CompiledModule) {
        todo!()
    }
}
