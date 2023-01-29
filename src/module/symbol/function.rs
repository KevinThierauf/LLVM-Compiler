use crate::module::modulepos::ModuleRange;
use crate::module::symbol::block::Block;
use crate::module::symbol::expr::Expr;
use crate::module::typeinfo::Type;

pub struct FunctionParameter {
    typeInfo: Type,
    parameterName: ModuleRange,
    defaultExpr: Expr,
}

pub struct Function {
    returnType: Type,
    functionName: ModuleRange,
    parameters: Vec<FunctionParameter>,
    functionBlock: Block,
}
