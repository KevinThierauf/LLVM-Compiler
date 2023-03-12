use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use hashbrown::HashSet;
use priority_queue::PriorityQueue;

use crate::module::modulepos::ModuleRange;
use crate::resolver::resolutionselector::ResolutionError;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
struct ConstraintInfo<T> {
    value: T,
    range: Vec<ModuleRange>,
}

impl<T> ConstraintInfo<T> {
    fn new(value: T, range: ModuleRange) -> Self {
        return Self {
            range: vec![range],
            value,
        };
    }

    fn addRange(&mut self, range: ModuleRange) {
        self.range.push(range);
    }
}

impl<T> Borrow<T> for ConstraintInfo<T> {
    fn borrow(&self) -> &T {
        return &self.value;
    }
}

impl<T: PartialOrd> PartialOrd<Self> for ConstraintInfo<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.value.partial_cmp(&other.value);
    }
}

impl<T: Ord> Ord for ConstraintInfo<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.value.cmp(&other.value);
    }
}

impl<T: PartialEq> PartialEq<Self> for ConstraintInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value.eq(&other.value);
    }
}

impl<T: Eq> Eq for ConstraintInfo<T> {}

impl<T: Hash> Hash for ConstraintInfo<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return self.value.hash(state);
    }
}

pub struct ResolutionConstraintSolver {
    requiredTypes: HashSet<ConstraintInfo<Type>>,
    subsetTypes: Option<Vec<ConstraintInfo<Type>>>,
    subsetVec: Vec<(Vec<Type>, ModuleRange)>,
    excluded: HashSet<ConstraintInfo<Type>>,
    priorityQueue: PriorityQueue<Type, u16>,
}

impl ResolutionConstraintSolver {
    pub fn new() -> Self {
        return Self {
            requiredTypes: HashSet::new(),
            subsetTypes: None,
            subsetVec: Vec::new(),
            excluded: HashSet::new(),
            priorityQueue: Default::default(),
        };
    }

    // must be one of
    pub fn setSubsetOrdered(&mut self, options: &Vec<Type>, range: ModuleRange) {
        fn checkOrdered(subset: &Vec<Type>) -> bool {
            let mut iter = subset.iter();
            let mut prev = iter.next();
            let mut next = iter.next();

            loop {
                match (prev, next) {
                    (Some(prevValue), Some(nextValue)) => {
                        if let Ordering::Less | Ordering::Equal = prevValue.cmp(nextValue) {
                            prev = Some(nextValue);
                            next = iter.next();
                        } else {
                            return false;
                        }
                    }
                    (_, _) => return true,
                }
            }
        }

        debug_assert!(checkOrdered(options), "options vector is not ordered");

        self.subsetVec.push((options.to_owned(), range.to_owned()));
        if let Some(subset) = self.subsetTypes.take() {
            debug_assert!(checkOrdered(&subset.iter().map(|constraint| constraint.value.to_owned()).collect()), "subset vector is not ordered");

            let mut subsetIter = subset.into_iter();
            let mut optionIter = options.iter();

            let mut nextSubset = subsetIter.next();
            let mut nextOption = optionIter.next();

            let mut optionVec = Vec::new();

            loop {
                let nextSubsetValue = if let Some(next) = &mut nextSubset {
                    next
                } else {
                    break;
                };

                let nextOptionValue = if let Some(next) = nextOption {
                    next
                } else {
                    break;
                };

                match nextSubsetValue.value.cmp(nextOptionValue) {
                    Ordering::Less => nextSubset = subsetIter.next(),
                    Ordering::Greater => nextOption = optionIter.next(),
                    Ordering::Equal => {
                        nextSubsetValue.addRange(range.to_owned());
                        optionVec.push(nextSubset.unwrap());

                        nextSubset = subsetIter.next();
                        nextOption = optionIter.next();
                    }
                }
            }

            self.subsetTypes = Some(optionVec);
        } else {
            self.subsetTypes = Some(options.iter().map(|v| ConstraintInfo::new(v.to_owned(), range.to_owned())).collect());
        }
    }

    // must be
    pub fn setForced(&mut self, value: &Type, range: ModuleRange) {
        self.requiredTypes.insert(ConstraintInfo::new(value.to_owned(), range));
    }

    // must not be
    pub fn setExcluded(&mut self, option: &Type, range: ModuleRange) {
        self.excluded.insert(ConstraintInfo::new(option.to_owned(), range));
    }

    pub fn isExcluded(&self, value: &Type) -> bool {
        return self.excluded.contains(value);
    }

    // in case of ambiguity, set priority
    pub fn setPriority(&mut self, value: &Type, priority: u16) {
        self.priorityQueue.push(value.to_owned(), priority);
    }

    pub fn getPriority(&self, value: &Type) -> u16 {
        return self.priorityQueue.get_priority(value).map(|v| *v).unwrap_or_default();
    }

    fn getSelectedType(&self) -> Result<Type, Vec<ResolutionError>> {
        debug_assert!(self.requiredTypes.is_empty());

        let mut priorityIter = self.priorityQueue.iter();
        let mut nextValue = priorityIter.next();
        let mut currentPriorityVec: Vec<Type> = Vec::new();
        let mut currentPriority = 0;

        while let Some((value, priority)) = nextValue {
            if currentPriorityVec.is_empty() {
                currentPriority = *priority;
            } else if currentPriority != *priority {
                break;
            }
            if !self.isExcluded(value) {
                currentPriorityVec.push(value.to_owned());
            }
            nextValue = priorityIter.next();
        }

        match currentPriorityVec.len() {
            0 => {}
            1 => return Ok(currentPriorityVec[0].to_owned()),
            _ => return Err(vec![ResolutionError::Ambiguous(currentPriorityVec)]),
        };

        let mut optionVec = Vec::new();
        let mut errorVec = Vec::new();

        if let Some(subsetTypes) = &self.subsetTypes {
            for typeInfo in subsetTypes {
                if let Some(err) = self.getExcludedRange(&typeInfo.value) {
                    errorVec.push(ResolutionError::Excluded { selected: typeInfo.value.to_owned(), excludedRange: err })
                } else {
                    optionVec.push(&typeInfo.value);
                }
            }
        }

        return match optionVec.len() {
            0 => {
                errorVec.push(if self.subsetTypes.is_some() || !self.priorityQueue.is_empty() {
                    ResolutionError::Eliminated
                } else {
                    ResolutionError::Unconstrained
                });
                Err(errorVec)
            }
            1 => Ok(optionVec[0].to_owned()),
            _ => Err(vec![ResolutionError::Ambiguous(optionVec.into_iter().map(|v| v.to_owned()).collect())])
        };
    }

    fn getExcludedRange(&self, typeInfo: &Type) -> Option<Vec<ModuleRange>> {
        return self.excluded.get(typeInfo).map(|constraint| constraint.range.to_owned());
    }

    fn getSubsetExcludedRange(&self, typeInfo: &Type) -> Vec<ModuleRange> {
        let mut vec = Vec::new();

        for (typeVec, range) in &self.subsetVec {
            // assumes typeVec is sorted
            if typeVec.binary_search(typeInfo).is_err() {
                vec.push(range.to_owned());
            }
        }

        return vec;
    }

    pub fn take(self) -> Result<Type, Vec<ResolutionError>> {
        return match self.requiredTypes.len() {
            0 => self.getSelectedType(),
            1 => {
                let typeInfo = self.requiredTypes.iter().next().unwrap();

                if let Some(excluded) = self.getExcludedRange(&typeInfo.value) {
                    Err(vec![ResolutionError::ForcedExcluded {
                        forced: typeInfo.value.to_owned(),
                        forcedRange: typeInfo.range.to_owned(),
                        excludedRange: excluded,
                    }])
                } else {
                    if let Some(subsetTypes) = &self.subsetTypes {
                        if subsetTypes.binary_search(typeInfo).is_ok() {
                            Ok(typeInfo.value.to_owned())
                        } else {
                            Err(vec![ResolutionError::ForcedSubset {
                                forced: typeInfo.value.to_owned(),
                                forcedRange: typeInfo.range.to_owned(),
                                excludedRange: self.getSubsetExcludedRange(&typeInfo.value),
                            }])
                        }
                    } else {
                        Ok(typeInfo.value.to_owned())
                    }
                }
            }
            _ => Err(vec![ResolutionError::Conflict(self.requiredTypes.into_iter().map(|v| (v.value, v.range)).collect())])
        };
    }
}

#[cfg(test)]
mod test {
    use crate::module::Module;
    use crate::module::modulepos::ModuleRange;
    use crate::resolver::resolutionselector::resolutionconstraintsolver::ResolutionConstraintSolver;
    use crate::resolver::resolutionselector::resolutionerror::ResolutionError;
    use crate::resolver::typeinfo::primitive::boolean::BOOLEAN_TYPE;
    use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
    use crate::resolver::typeinfo::Type;

    thread_local! {
        static RANGE: ModuleRange = Module::newFrom(Vec::new()).getModuleRange(0..0);
    }

    fn getRange() -> ModuleRange {
        return RANGE.with(|v| v.to_owned());
    }

    fn sortError(error: &mut ResolutionError) {
        fn sortInner(vec: &mut Vec<(Type, Vec<ModuleRange>)>) {
            vec.sort();
            for (_, v) in vec {
                v.sort();
            }
        }

        match error {
            ResolutionError::Conflict(vec) => sortInner(vec),
            ResolutionError::ForcedExcluded { forced: _, forcedRange, excludedRange } => {
                forcedRange.sort();
                excludedRange.sort();
            }
            ResolutionError::ForcedSubset { forced: _, forcedRange, excludedRange } => {
                forcedRange.sort();
                excludedRange.sort();
            }
            ResolutionError::Excluded { selected: _, excludedRange } => excludedRange.sort(),
            ResolutionError::Ambiguous(vec) => vec.sort(),
            ResolutionError::Eliminated => {}
            ResolutionError::Unconstrained => {}
        }
    }

    fn assertEq(solver: ResolutionConstraintSolver, expected: Result<Type, Vec<ResolutionError>>) {
        let result = solver.take();

        match (result, expected) {
            (Ok(result), Ok(expected)) => assert_eq!(result, expected),
            (Ok(result), Err(expected)) => panic!("expected error {:#?}, found {:?}", expected, result),
            (Err(result), Ok(expected)) => panic!("expected {:?}, found error {:#?}", expected, result),
            (Err(mut result), Err(mut expected)) => {
                result.sort();
                expected.sort();

                result.iter_mut().for_each(sortError);
                expected.iter_mut().for_each(sortError);

                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn testForced() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setForced(&BOOLEAN_TYPE, getRange());
        assertEq(solver, Ok(BOOLEAN_TYPE.to_owned()));
    }

    #[test]
    fn testForcedConflict() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setForced(&BOOLEAN_TYPE, getRange());
        solver.setForced(&CHARACTER_TYPE, getRange());
        assertEq(solver, Err(vec![ResolutionError::Conflict(vec![(BOOLEAN_TYPE.to_owned(), vec![getRange()]), (CHARACTER_TYPE.to_owned(), vec![getRange()])])]));
    }

    #[test]
    fn testForcedConstraint() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setForced(&BOOLEAN_TYPE, getRange());
        solver.setSubsetOrdered(&Vec::new(), getRange());
        assertEq(solver, Err(vec![ResolutionError::ForcedSubset {
            forced: BOOLEAN_TYPE.to_owned(),
            forcedRange: vec![getRange()],
            excludedRange: vec![getRange()],
        }]));
    }

    #[test]
    fn testPriorityEliminated() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setPriority(&BOOLEAN_TYPE, 1);
        solver.setExcluded(&BOOLEAN_TYPE, getRange());
        assert!(solver.isExcluded(&BOOLEAN_TYPE));
        assertEq(solver, Err(vec![ResolutionError::Eliminated]));
    }

    #[test]
    fn testEliminated() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setSubsetOrdered(&Vec::new(), getRange());
        assertEq(solver, Err(vec![ResolutionError::Eliminated]));
    }

    #[test]
    fn testUnconstrained() {
        let solver = ResolutionConstraintSolver::new();
        assertEq(solver, Err(vec![ResolutionError::Unconstrained]));
    }

    #[test]
    fn testUnconstrainedExclude() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setExcluded(&BOOLEAN_TYPE, getRange());
        assert!(solver.isExcluded(&BOOLEAN_TYPE));
        assertEq(solver, Err(vec![ResolutionError::Unconstrained]));
    }

    #[test]
    fn testSubsetExcluded() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setExcluded(&BOOLEAN_TYPE, getRange());
        solver.setSubsetOrdered(&vec![BOOLEAN_TYPE.to_owned()], getRange());
        assertEq(solver, Err(vec![
            ResolutionError::Excluded {
                selected: BOOLEAN_TYPE.to_owned(),
                excludedRange: vec![getRange()],
            },
            ResolutionError::Eliminated,
        ]));
    }

    #[test]
    fn testPriority() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setPriority(&BOOLEAN_TYPE, 1);
        assertEq(solver, Ok(BOOLEAN_TYPE.to_owned()));
    }

    #[test]
    fn testSubset() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setSubsetOrdered(&vec![BOOLEAN_TYPE.to_owned()], getRange());
        assertEq(solver, Ok(BOOLEAN_TYPE.to_owned()));
    }

    #[test]
    fn testPrioritySubset() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setPriority(&BOOLEAN_TYPE, 1);
        let mut vec = vec![BOOLEAN_TYPE.to_owned(), CHARACTER_TYPE.to_owned()];
        vec.sort();
        solver.setSubsetOrdered(&vec, getRange());
        assertEq(solver, Ok(BOOLEAN_TYPE.to_owned()));
    }

    #[test]
    fn testExcludedSubset() {
        let mut solver = ResolutionConstraintSolver::new();
        solver.setExcluded(&CHARACTER_TYPE, getRange());
        let mut vec = vec![BOOLEAN_TYPE.to_owned(), CHARACTER_TYPE.to_owned()];
        vec.sort();
        solver.setSubsetOrdered(&vec, getRange());
        assertEq(solver, Ok(BOOLEAN_TYPE.to_owned()));
    }
}
