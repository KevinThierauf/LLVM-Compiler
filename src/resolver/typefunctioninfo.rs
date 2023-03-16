use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;

#[derive(Debug)]
pub struct TypeFunctionInfo {
    functionVec: HashMap<String, Function>,
}

impl TypeFunctionInfo {
    pub fn new() -> Self {
        return Self {
            functionVec: HashMap::new(),
        }
    }

    pub fn addFunction(&mut self, function: Function) -> Result<(), ResolutionError> {
        return match self.functionVec.entry(function.getFunctionName().to_owned()) {
            Entry::Occupied(entry) => Err(ResolutionError::ConflictingFunction(function, entry.get().to_owned())),
            Entry::Vacant(mut v) => {
                v.insert(function);
                Ok(())
            }
        };
    }
}
