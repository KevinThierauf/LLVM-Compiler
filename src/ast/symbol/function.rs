use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::SymbolType;
use crate::module::Keyword;
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::ast::visibility::Visibility;

#[derive(Debug)]
pub struct FunctionParameter {
    pub typeName: ModulePos,
    pub parameterName: ModulePos,
    pub defaultExpr: Option<Expr>,
}

#[derive(Debug)]
pub enum FunctionAttribute {
    Static,
}

impl FunctionAttribute {
    pub fn fromKeyword(keyword: Keyword) -> Option<FunctionAttribute> {
        return match keyword {
            Keyword::Static => Some(FunctionAttribute::Static),
            _ => None
        };
    }
}

#[derive(Debug)]
pub struct FunctionDefinitionSym {
    pub range: ModuleRange,
    pub attributeVec: Vec<FunctionAttribute>,
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
