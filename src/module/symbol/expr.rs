pub mod ifexpr;
pub mod forexpr;
pub mod breakexpr;
pub mod label;
pub mod looptype;
pub mod resolution;

use crate::module::modulepos::ModuleRange;
use crate::module::symbol::expr::resolution::{ResolutionConstraintType, ResolutionConstraintSolver, ResolutionError};
use crate::module::typeinfo::Type;

pub trait ExprType {
    fn getRange(&self) -> &ModuleRange;
    fn getResolutionSolver(&self) -> &ResolutionConstraintSolver;
    fn getResolutionSolverMut(&mut self) -> &mut ResolutionConstraintSolver;
}

pub type Expr = Box<dyn ExprType>;
