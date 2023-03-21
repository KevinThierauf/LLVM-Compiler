use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct WhileLoop {
    pub range: ModuleRange,
    pub condition: Expr,
    pub symbol: Box<Symbol>,
    pub label: Option<Label>,
}

impl SymbolType for WhileLoop {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl LoopType for WhileLoop {
    fn getLabel(&self) -> Option<&Label> {
        return self.label.as_ref();
    }
}
