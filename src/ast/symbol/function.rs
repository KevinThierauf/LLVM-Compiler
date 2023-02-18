use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::module::visibility::Visibility;

#[derive(Debug)]
pub struct FunctionParameter {
    pub typeName: ModulePos,
    pub parameterName: ModulePos,
    pub defaultExpr: Option<Expr>,
}

#[derive(Debug)]
pub struct FunctionDefinitionSym {
    pub range: ModuleRange,
    pub returnType: ModulePos,
    pub functionName: ModulePos,
    pub parameters: Vec<FunctionParameter>,
    pub functionBlock: BlockSym,
    pub visibility: Visibility,
}

impl SymbolType for FunctionDefinitionSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
