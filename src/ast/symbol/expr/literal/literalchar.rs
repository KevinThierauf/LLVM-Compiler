use std::fmt::{Debug, Formatter};

use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

pub struct LiteralChar {
    pub range: ModuleRange,
    pub value: u32,
}

impl Debug for LiteralChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "char({})", self.value);
    }
}

impl ExprType for LiteralChar {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl SymbolType for LiteralChar {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralChar {}