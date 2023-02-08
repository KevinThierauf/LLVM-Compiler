use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;
use crate::module::visibility::Visibility;

pub struct FunctionParameter {
    typeInfo: Type,
    parameterName: ModuleRange,
    defaultExpr: Option<Expr>,
}

pub struct FunctionDefinitionSym {
    range: ModuleRange,
    returnType: Type,
    functionName: ModuleRange,
    parameters: Vec<FunctionParameter>,
    functionBlock: BlockSym,
    visibility: Visibility,
}

impl SymbolType for FunctionDefinitionSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
