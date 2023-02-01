use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use hashbrown::HashSet;
use priority_queue::PriorityQueue;

use crate::module::modulepos::ModuleRange;
use crate::module::resolutionselector::resolutionconstraint::ResolutionConstraintType;
use crate::module::resolutionselector::ResolutionError;
use crate::module::typeinfo::Type;

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
    excluded: HashSet<ConstraintInfo<Type>>,
    children: Vec<Vec<ResolutionConstraintSolver>>,
    priorityQueue: PriorityQueue<Type, u16>,
}

impl ResolutionConstraintSolver {
    pub fn new() -> Self {
        return Self {
            requiredTypes: HashSet::new(),
            subsetTypes: None,
            excluded: HashSet::new(),
            children: Vec::new(),
            priorityQueue: Default::default(),
        };
    }

    // must be one of
    pub fn setSubset(&mut self, options: &Vec<Type>, range: ModuleRange) {
        if let Some(subset) = self.subsetTypes.take() {
            fn checkOrdered(subset: &Vec<ConstraintInfo<Type>>) -> bool {
                let mut iter = subset.iter();
                let mut prev = iter.next();
                let mut next = iter.next();

                loop {
                    match (prev, next) {
                        (Some(prevValue), Some(nextValue)) => {
                            if let Ordering::Less | Ordering::Equal = prevValue.value.cmp(&nextValue.value) {
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

            debug_assert!(checkOrdered(&subset), "subset vector is not ordered");

            let mut subsetIter = subset.into_iter();
            let mut optionIter = options.iter();

            let mut nextSubset = subsetIter.next();
            let mut nextOption = optionIter.next();

            let mut optionVec = Vec::new();

            loop {
                let mut nextSubsetValue = if let Some(next) = &mut nextSubset {
                    next
                } else {
                    break;
                };

                let mut nextOptionValue = if let Some(next) = nextOption {
                    next
                } else {
                    break;
                };

                match nextSubsetValue.value.cmp(nextOptionValue) {
                    Ordering::Less => nextSubset = subsetIter.next(),
                    Ordering::Greater => nextOption = optionIter.next(),
                    Ordering::Equal => {
                        nextSubsetValue.range.push(range.to_owned());
                        optionVec.push(nextSubset.unwrap());

                        nextSubset = subsetIter.next();
                        nextOption = optionIter.next();
                    }
                }
            }

            self.subsetTypes = Some(optionVec);
        } else {
            let mut vec: Vec<ConstraintInfo<Type>> = options.iter().map(|v| ConstraintInfo::new(v.to_owned(), range.to_owned())).collect();
            vec.sort_by(|a, b| a.value.cmp(&b.value));
            self.subsetTypes = Some(vec);
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
                return match currentPriorityVec.len() {
                    1 => Ok(currentPriorityVec[0].to_owned()),
                    _ => Err(vec![ResolutionError::Ambiguous(currentPriorityVec)]),
                };
            }
            if !self.isExcluded(value) {
                currentPriorityVec.push(value.to_owned());
            }
            nextValue = priorityIter.next();
        }

        let mut optionVec = Vec::new();
        let mut errorVec = Vec::new();

        if let Some(subsetTypes) = &self.subsetTypes {
            for typeInfo in subsetTypes {
                if let Some(err) = self.getExcludedRange(&typeInfo.value) {
                    errorVec.push(ResolutionError::Excluded(err))
                } else {
                    optionVec.push(&typeInfo.value);
                }
            }
        }

        return match optionVec.len() {
            0 => {
                errorVec.push(ResolutionError::Eliminated);
                Err(errorVec)
            },
            1 => Ok(optionVec[0].to_owned()),
            _ => Err(vec![ResolutionError::Ambiguous(optionVec.into_iter().map(|v| v.to_owned()).collect())])
        };
    }

    fn getExcludedRange(&self, typeInfo: &Type) -> Option<Vec<ModuleRange>> {
        return self.excluded.get(typeInfo).map(|constraint| constraint.range.to_owned());
    }

    pub fn take(self) -> Result<Type, Vec<ResolutionError>> {
        return match self.requiredTypes.len() {
            0 => self.getSelectedType(),
            1 => {
                let typeInfo = self.requiredTypes.iter().next().unwrap();

                if let Some(excluded) = self.getExcludedRange(&typeInfo.value) {
                    Err(vec![ResolutionError::Excluded(excluded)])
                } else {
                    if let Some(subsetTypes) = self.subsetTypes {
                        if subsetTypes.binary_search(typeInfo).is_ok() {
                            Ok(typeInfo.value.to_owned())
                        } else {
                            Err(vec![ResolutionError::ForcedConstraint(typeInfo.value.to_owned(), subsetTypes.into_iter().map(|constraint| constraint.range).flatten().collect())])
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
