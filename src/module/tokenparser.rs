use crate::module::Module;

pub(in super) struct TokenParser<'a> {
    module: &'a mut Module,
}

impl<'a> TokenParser<'a> {
    pub(in super) fn new(module: &'a mut Module) -> Self {
        return Self {
            module,
        }
    }

    pub(in super) fn parse(mut self) {
    }
}