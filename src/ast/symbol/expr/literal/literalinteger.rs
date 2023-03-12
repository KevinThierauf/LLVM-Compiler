use std::fmt::{Debug, Formatter};

use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::Type;

pub struct LiteralInteger {
    pub range: ModuleRange,
    pub value: i64,
}

impl Debug for LiteralInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "int({})", self.value);
    }
}

impl ExprType for LiteralInteger {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::LiteralInteger(*self);
    }
}

impl SymbolType for LiteralInteger {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralInteger {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}