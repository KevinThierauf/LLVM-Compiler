use std::fmt::{Debug, Formatter};

use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

pub struct LiteralFloat {
    pub range: ModuleRange,
    pub value: f64,
}

impl Debug for LiteralFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "float({})", self.value);
    }
}

impl ExprType for LiteralFloat {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl SymbolType for LiteralFloat {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralFloat {
}