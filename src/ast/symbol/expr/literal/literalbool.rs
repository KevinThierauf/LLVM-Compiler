use std::fmt::{Debug, Formatter};
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::ast::typeinfo::Type;
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
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::LiteralBool(*self);
    }
}

impl LiteralType for LiteralBool {
    fn getLiteralType(&self) -> Type {
        return BOOLEAN_TYPE.to_owned();
    }
}
