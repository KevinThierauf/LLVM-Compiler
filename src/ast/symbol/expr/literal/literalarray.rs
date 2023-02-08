use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

pub struct LiteralArray {
    range: ModuleRange,
    baseType: Type,
    arrayLength: usize,
}

impl ExprType for LiteralArray {}

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
