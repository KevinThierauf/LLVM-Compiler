use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::ast::symbol::SymbolType;
use crate::ast::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::ast::typeinfo::Type;

pub struct LiteralBool {
    range: ModuleRange,
    value: bool,
}

impl LiteralBool {
    pub fn new(range: ModuleRange, value: bool) -> Self {
        return Self {
            range,
            value,
        };
    }
}

impl SymbolType for LiteralBool {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for LiteralBool {
}

impl LiteralType for LiteralBool {
    fn getLiteralType(&self) -> Type {
        return BOOLEAN_TYPE.to_owned();
    }
}
