use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

pub struct LiteralString {
    range: ModuleRange,
    strRange: ModuleRange,
}

impl ExprType for LiteralString {}

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