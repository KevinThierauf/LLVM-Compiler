use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::LoopType;
use crate::ast::symbol::SymbolType;
use crate::module::modulepos::ModuleRange;

#[derive(Debug)]
pub struct WhileLoop {
    range: ModuleRange,
    conditional: Expr,
    label: Option<Label>,
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
