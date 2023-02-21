use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::typeinfo::Type;
use crate::module::FileRange;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct LiteralString {
    pub range: ModuleRange,
    pub fileRange: FileRange,
}

impl ExprType for LiteralString {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::LiteralString(*self);
    }
}

impl SymbolType for LiteralString {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralString {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}