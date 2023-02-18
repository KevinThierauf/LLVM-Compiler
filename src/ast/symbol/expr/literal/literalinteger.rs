use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

#[derive(Debug)]
pub struct LiteralInteger {
    pub range: ModuleRange,
}

impl ExprType for LiteralInteger {}

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