use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::ResolutionSelector;

pub mod ifexpr;
pub mod forexpr;
pub mod breakexpr;
pub mod label;
pub mod looptype;

pub trait ExprType {
    fn getRange(&self) -> &ModuleRange;
    fn getResolutionSolver(&self) -> &ResolutionSelector;
    fn getResolutionSolverMut(&mut self) -> &mut ResolutionSelector;
}

pub type Expr = Box<dyn ExprType>;
