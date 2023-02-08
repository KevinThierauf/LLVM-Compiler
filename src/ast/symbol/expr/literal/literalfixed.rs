use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

pub struct LiteralFixed {
    range: ModuleRange,
}

impl ExprType for LiteralFixed {}

impl SymbolType for LiteralFixed {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralFixed {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}