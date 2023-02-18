use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

#[derive(Debug)]
pub struct LiteralFloat {
    pub range: ModuleRange,
}

impl ExprType for LiteralFloat {}

impl SymbolType for LiteralFloat {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralFloat {
    fn getLiteralType(&self) -> Type {
        todo!()
    }
}