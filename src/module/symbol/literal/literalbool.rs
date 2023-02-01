use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::resolutionconstraint::ResolutionConstraintType;
use crate::module::resolutionselector::ResolutionSelector;
use crate::module::symbol::expr::ExprType;
use crate::module::symbol::literal::LiteralType;
use crate::module::typeinfo::primitive::boolean::BOOLEAN_TYPE;

pub struct LiteralBool {
    range: ModuleRange,
    value: bool,
    resolutionSolver: ResolutionSelector,
}

impl LiteralBool {
    pub fn new(range: ModuleRange, value: bool) -> Self {
        let rangeCopy = range.clone();
        return Self {
            range,
            value,
            resolutionSolver: ResolutionSelector::newFrom(rangeCopy, ResolutionConstraintType::Implicit(BOOLEAN_TYPE.to_owned())),
        };
    }
}

impl ExprType for LiteralBool {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }

    fn getResolutionSelector(&self) -> &ResolutionSelector {
        return &self.resolutionSolver;
    }

    fn getResolutionSelectorMut(&mut self) -> &mut ResolutionSelector {
        return &mut self.resolutionSolver;
    }
}

impl LiteralType for LiteralBool {
    //
}