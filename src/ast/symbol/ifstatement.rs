use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

pub struct ElseSym {
    expr: Expr
}

pub struct IfSym {
    expr: Expr,
    condition: Expr,
    range: ModuleRange,
    elseExpr: Option<ElseSym>,
}

impl SymbolType for IfSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
