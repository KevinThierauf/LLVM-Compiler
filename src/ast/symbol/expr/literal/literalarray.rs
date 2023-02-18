use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

#[derive(Debug)]
pub struct LiteralArray {
    pub range: ModuleRange,
    pub exprVec: Vec<Expr>,
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
