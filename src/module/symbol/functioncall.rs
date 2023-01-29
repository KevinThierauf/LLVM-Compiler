use crate::module::modulepos::ModuleRange;
use crate::module::symbol::expr::Expr;

pub struct FunctionCall {
    functionName: ModuleRange,
    argVec: Vec<Expr>,
}