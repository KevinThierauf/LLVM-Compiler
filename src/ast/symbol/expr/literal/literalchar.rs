use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::ast::typeinfo::Type;

pub struct LiteralChar {
    range: ModuleRange,
    character: u32,
}

impl ExprType for LiteralChar {}

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