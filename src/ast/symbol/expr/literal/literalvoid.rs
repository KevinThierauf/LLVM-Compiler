use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct LiteralVoid {
    pub range: ModuleRange,
}

impl ExprType for LiteralVoid {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl SymbolType for LiteralVoid {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralVoid {
}
