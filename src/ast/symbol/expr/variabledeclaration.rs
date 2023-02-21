use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::module::modulepos::{ModulePos, ModuleRange};

#[derive(Debug)]
pub struct VariableDeclarationExpr {
    pub range: ModuleRange,
    pub variableName: ModulePos,
    pub explicitType: Option<ModulePos>,
}

impl SymbolType for VariableDeclarationExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for VariableDeclarationExpr {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::VariableDeclaration(*self);
    }
}
