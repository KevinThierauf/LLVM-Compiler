use crate::module::symbol::expr::Expr;

pub struct ElseExpr {
    expr: Expr
}

pub struct IfExpr {
    expr: Expr,
    condition: Expr,
    elseExpr: Option<ElseExpr>,
}
