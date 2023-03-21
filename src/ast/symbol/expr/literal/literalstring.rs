use crate::ast::symbol::SymbolType;
use crate::ast::symbol::expr::ExprType;
use crate::ast::symbol::expr::literal::LiteralType;
use crate::module::FileRange;
use crate::module::modulepos::ModuleRange;
use crate::resolver::typeinfo::string::STRING_TYPE;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
pub struct LiteralString {
    pub range: ModuleRange,
    pub fileRange: FileRange,
}

impl ExprType for LiteralString {
    fn getSymbolType(&self) -> &dyn SymbolType {
        return self;
    }
}

impl SymbolType for LiteralString {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LiteralType for LiteralString {
    fn getLiteralType(&self) -> Type {
        return STRING_TYPE.to_owned();
    }
}