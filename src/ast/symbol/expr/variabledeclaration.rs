use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::Type;

#[derive(Debug)]
pub struct VariableDeclarationExpr {
    range: ModuleRange,
    variableName: ModuleRange,
    explicitType: Option<Type>,
}

impl SymbolType for VariableDeclarationExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for VariableDeclarationExpr {
}
