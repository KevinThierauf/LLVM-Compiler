use std::fmt::{Debug, Formatter};
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::ast::typeinfo::Type;
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
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::LiteralChar(*self);
    }
}

impl SymbolType for LiteralChar {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralChar {
    fn getLiteralType(&self) -> Type {
        return CHARACTER_TYPE.to_owned();
    }
}