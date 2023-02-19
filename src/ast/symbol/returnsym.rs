use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ReturnSym {
    range: ModuleRange,
    value: Option<Expr>,
}

impl SymbolType for ReturnSym {
    fn getRange(&self) -> &ModuleRange {
        todo!()
    }
}
