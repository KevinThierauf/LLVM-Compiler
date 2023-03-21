use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct LiteralTuple {
    pub range: ModuleRange,
    pub exprVec: Vec<Expr>,
}

impl ExprType for LiteralTuple {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

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
