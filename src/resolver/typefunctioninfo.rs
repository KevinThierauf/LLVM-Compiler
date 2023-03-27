use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;

#[derive(Debug)]
pub struct TypeFunctionInfo {
    functionMap: HashMap<String, Function>,
}

impl TypeFunctionInfo {
    pub fn new() -> Self {
        return Self {
            functionMap: HashMap::new(),
        };
    }

    pub fn addFunction(&mut self, function: Function) -> Result<(), ResolutionError> {
        return match self.functionMap.entry(function.getFunctionName().to_owned()) {
            Entry::Occupied(entry) => Err(ResolutionError::ConflictingFunction(function, entry.get().to_owned())),
            Entry::Vacant(v) => {
                v.insert(function);
                Ok(())
            }
        };
    }

    pub fn getFunction(&self, name: &str) -> Option<Function> {
        return self.functionMap.get(name).map(|v| v.to_owned());
    }
}
