use std::fmt::{Debug, Formatter};

use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::literal::LiteralType;
use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::Type;

pub struct LiteralArray {
    pub range: ModuleRange,
    pub exprVec: Vec<Expr>,
}

impl Debug for LiteralArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.exprVec);
    }
}

impl ExprType for LiteralArray {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::LiteralArray(*self);
    }
}

impl SymbolType for LiteralArray {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralArray {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}
