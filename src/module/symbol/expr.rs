use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::ResolutionSelector;

pub mod ifexpr;
pub mod breakexpr;
pub mod looptype;

pub trait ExprType {
    fn getRange(&self) -> &ModuleRange;
    fn getResolutionSelector(&self) -> &ResolutionSelector;
    fn getResolutionSelectorMut(&mut self) -> &mut ResolutionSelector;
}

pub type Expr = Box<dyn ExprType>;
