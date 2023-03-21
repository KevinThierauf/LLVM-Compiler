use std::fmt::{Debug, Formatter};

use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::Type;

pub struct LiteralInteger {
    pub range: ModuleRange,
    pub value: i64,
}

impl Debug for LiteralInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "int({})", self.value);
    }
}

impl ExprType for LiteralInteger {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl SymbolType for LiteralInteger {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralInteger {
    fn getLiteralType(&self) -> Type {
        return INTEGER_TYPE.to_owned();
    }
}
