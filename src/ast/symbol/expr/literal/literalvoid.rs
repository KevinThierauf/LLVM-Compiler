use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;
use crate::ast::typeinfo::void::VOID_TYPE;

#[derive(Debug)]
pub struct LiteralVoid {
    pub range: ModuleRange,
}

impl ExprType for LiteralVoid {}

impl SymbolType for LiteralVoid {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralVoid {
    fn getLiteralType(&self) -> Type {
        return VOID_TYPE.to_owned();
    }
}
