use crate::module::modulepos::ModuleRange;
use crate::module::symbol::expr::ExprType;
use crate::module::symbol::expr::resolution::{ResolutionConstraintType, ResolutionConstraintSolver, ResolutionError};
use crate::module::symbol::literal::LiteralType;
use crate::module::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::module::typeinfo::Type;
use crate::source::token::Keyword;

pub struct LiteralBool {
    range: ModuleRange,
    value: bool,
    resolutionSolver: ResolutionConstraintSolver,
}

impl LiteralBool {
    pub fn new(range: ModuleRange, value: bool) -> Self {
        let rangeCopy = range.clone();
        return Self {
            range,
            value,
            resolutionSolver: ResolutionConstraintSolver::newFrom(rangeCopy, ResolutionConstraintType::Implicit(BOOLEAN_TYPE.to_owned())),
        };
    }
}

impl ExprType for LiteralBool {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }

    fn getResolutionSolver(&self) -> &ResolutionConstraintSolver {
        return &self.resolutionSolver;
    }

    fn getResolutionSolverMut(&mut self) -> &mut ResolutionConstraintSolver {
        return &mut self.resolutionSolver;
    }
}

impl LiteralType for LiteralBool {
    //
}