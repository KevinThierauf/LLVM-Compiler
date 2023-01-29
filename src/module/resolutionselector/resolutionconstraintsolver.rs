use hashbrown::HashSet;
use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::resolutionconstraint::{ResolutionConstraint, ResolutionConstraintType};
use crate::module::resolutionselector::ResolutionError;
use crate::module::typeinfo::Type;

pub struct ResolutionConstraintSolver {
    requiredTypes: HashSet<Type>,
    subsetTypes: Option<HashSet<Type>>,
    excluded: HashSet<Type>,
    children: Vec<Vec<ResolutionConstraintSolver>>,
}

impl ResolutionConstraintSolver {
    pub fn new() -> Self {
        return Self {
            requiredTypes: HashSet::new(),
            subsetTypes: None,
            excluded: HashSet::new(),
            children: Vec::new(),
        };
    }

    // must be one of
    pub fn setSubset(&mut self, options: &HashSet<Type>) {
        if let Some(subset) = &mut self.subsetTypes {
            self.subsetTypes = Some(subset.intersection(&options).map(|v| v.to_owned()).collect());
        } else {
            self.subsetTypes = Some(options.to_owned());
        }
    }

    // must be
    pub fn setForced(&mut self, value: &Type) {
        self.requiredTypes.insert(value.to_owned());
    }

    // must not be
    pub fn setExcluded(&mut self, option: &Type) {
        self.excluded.insert(option.to_owned());
    }

    // in case of ambiguity, set priority
    pub fn setPriority(&mut self, value: &Type, priority: u16) {
        todo!()
    }

    // can be any of the following (must be at least one)
    pub fn setAnyOf(&mut self, range: &ModuleRange, options: &[ResolutionConstraintType]) {
        let mut childVec = Vec::new();

        for option in options {
            let mut selector = ResolutionConstraintSolver::new();
            option.resolve(range, &mut selector);
            childVec.push(selector);
        }

        self.children.push(childVec);
    }

    fn getSelectedType(self) -> Result<Type, Vec<ResolutionError>> {
        todo!()
    }

    fn isTypeAllowed(&self, typeInfo: &Type) -> Result<(), Vec<ResolutionConstraint>> {
        todo!()
    }

    pub fn take(self) -> Result<Type, Vec<ResolutionError>> {
        return match self.requiredTypes.len() {
            0 => self.getSelectedType(),
            1 => {
                let typeInfo = self.requiredTypes.iter().next().unwrap();
                if let Err(constraint) = self.isTypeAllowed(typeInfo) {
                    Err(vec![ResolutionError::ConstraintFailure(constraint)])
                } else {
                    Ok(typeInfo.to_owned())
                }
            }
            _ => Err(vec![ResolutionError::Conflict(self.requiredTypes.into_iter().collect())])
        };
    }
}
