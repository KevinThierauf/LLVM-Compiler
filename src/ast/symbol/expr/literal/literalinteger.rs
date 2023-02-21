use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::typeinfo::Type;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct LiteralInteger {
    pub range: ModuleRange,
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