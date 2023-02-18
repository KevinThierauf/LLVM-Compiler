use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct ElseSym {
    pub symbol: Symbol
}

#[derive(Debug)]
pub struct IfSym {
    pub symbol: Box<Symbol>,
    pub condition: Expr,
    pub range: ModuleRange,
    pub elseExpr: Option<Box<ElseSym>>,
}

impl SymbolType for IfSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
