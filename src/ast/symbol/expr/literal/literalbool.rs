use std::fmt::{Debug, Formatter};

use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

pub struct LiteralBool {
    pub range: ModuleRange,
    pub value: bool,
}

impl Debug for LiteralBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "bool({})", self.value);
    }
}

impl LiteralBool {
    pub fn new(range: ModuleRange, value: bool) -> Self {
        return Self {
            range,
            value,
        };
    }
}

impl SymbolType for LiteralBool {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for LiteralBool {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl LiteralType for LiteralBool {
}
