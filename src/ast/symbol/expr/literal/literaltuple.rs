use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

pub struct LiteralTuple {
    range: ModuleRange,
    values: Vec<Expr>,
}

impl ExprType for LiteralTuple {}

impl SymbolType for LiteralTuple {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralTuple {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}
