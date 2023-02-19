use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

use crate::ast::ASTError;
use crate::module::modulepos::{ModulePos, ModuleRange};

pub trait MatchType: Clone + Debug {
    type Value: Debug;

    fn getDescription(&self) -> String;
    fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError>;
}

#[derive(Debug)]
pub struct Match<T> {
    range: ModuleRange,
    value: T,
}

impl<T: Debug> Match<T> {
    pub fn new(range: ModuleRange, value: T) -> Self {
        return Self {
            range,
            value,
        };
    }

    pub fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }

    pub fn getValue(&self) -> &T {
        return &self.value;
    }

    pub fn take(self) -> (ModuleRange, T) {
        return (self.range, self.value);
    }
}

#[derive(Debug, Clone)]
pub struct OptionalMatch<T: MatchType>(T);

impl<T: MatchType> OptionalMatch<T> {
    pub fn new(value: T) -> Self {
        return Self {
            0: value,
        };
    }
}

impl<T: Debug + MatchType> MatchType for OptionalMatch<T> {
    type Value = Option<T::Value>;

    fn getDescription(&self) -> String {
        return format!("Optional({})", self.0.getDescription());
    }

    fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
        return Ok(if let Ok(matched) = self.0.getMatch(startPos.to_owned()) {
            let (range, value) = matched.take();
            Match::new(range, Some(value))
        } else {
            Match::new(startPos.getRangeWithLength(0), None)
        });
    }
}

pub fn getMatchFrom<S: Debug>(description: String, function: impl 'static + Clone + Fn(ModulePos) -> Result<Match<S>, ASTError>) -> impl MatchType<Value = S> {
    struct MatchImpl<F: 'static + Clone + Fn(ModulePos) -> Result<Match<S>, ASTError>, S> {
        description: String,
        function: F,
    }

    impl<F: 'static + Clone + Fn(ModulePos) -> Result<Match<S>, ASTError>, S> Debug for MatchImpl<F, S> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return writeln!(f, "getMatchFrom()");
        }
    }

    impl<F: 'static + Clone + Fn(ModulePos) -> Result<Match<S>, ASTError>, S> Clone for MatchImpl<F, S> {
        fn clone(&self) -> Self {
            return Self {
                description: self.description.to_owned(),
                function: self.function.to_owned(),
            };
        }
    }

    impl<F: 'static + Clone + Fn(ModulePos) -> Result<Match<S>, ASTError>, S: Debug> MatchType for MatchImpl<F, S> {
        type Value = S;

        fn getDescription(&self) -> String {
            return self.description.to_owned();
        }

        fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
            return (self.function)(startPos);
        }
    }

    return MatchImpl {
        description,
        function,
    };
}

pub fn getMappedMatch<S: Debug, T: MatchType>(matcher: T, function: impl 'static + Clone + Fn(ModuleRange, T::Value) -> Result<S, ASTError>) -> impl MatchType<Value = S> {
    struct MatchNested<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Result<S, ASTError>> {
        matcher: T,
        function: F,
    }

    impl<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Result<S, ASTError>> Debug for MatchNested<S, T, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return writeln!(f, "getMappedMatch({:?})", self.matcher);
        }
    }

    impl<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Result<S, ASTError>> Clone for MatchNested<S, T, F> {
        fn clone(&self) -> Self {
            return Self {
                matcher: self.matcher.to_owned(),
                function: self.function.to_owned(),
            };
        }
    }

    impl<S: Debug, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Result<S, ASTError>> MatchType for MatchNested<S, T, F> {
        type Value = S;

        fn getDescription(&self) -> String {
            return self.matcher.getDescription();
        }

        fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
            let matched = self.matcher.getMatch(startPos)?;
            let (range, value) = matched.take();
            return Ok(Match::new(range.to_owned(), (self.function)(range, value)?));
        }
    }

    return MatchNested {
        matcher,
        function,
    };
}

trait DynMatchOptionType<S: 'static> {
    fn getDescription(&self) -> String;
    fn getMatchValue(&self, startPos: ModulePos) -> Result<Match<S>, ASTError>;
    fn cloneDynamic(&self) -> Box<dyn DynMatchOptionType<S>>;
}

impl<S: 'static> Clone for Box<dyn DynMatchOptionType<S>> {
    fn clone(&self) -> Self {
        return self.cloneDynamic();
    }
}

struct DynMatchOption<S: 'static, T: 'static, F: 'static + MatchType<Value = T>, M: 'static + Clone + Fn(&ModuleRange, T) -> Result<S, ASTError>> {
    matchType: F,
    mappingFunction: M,
}

impl<S: 'static + Debug, T: 'static + Debug, F: 'static + MatchType<Value = T>, M: 'static + Clone + Fn(&ModuleRange, T) -> Result<S, ASTError>> DynMatchOptionType<S> for DynMatchOption<S, T, F, M> {
    fn getDescription(&self) -> String {
        return self.matchType.getDescription();
    }

    fn getMatchValue(&self, startPos: ModulePos) -> Result<Match<S>, ASTError> {
        let (range, v) = self.matchType.getMatch(startPos)?.take();
        let mappedValue = (self.mappingFunction)(&range, v)?;
        return Ok(Match::new(range, mappedValue));
    }

    fn cloneDynamic(&self) -> Box<dyn DynMatchOptionType<S>> {
        return Box::new(Self {
            matchType: self.matchType.to_owned(),
            mappingFunction: self.mappingFunction.to_owned(),
        });
    }
}

pub struct MatchOption<S: 'static> {
    matchOption: Box<dyn DynMatchOptionType<S>>,
}

impl<S: 'static> Clone for MatchOption<S> {
    fn clone(&self) -> Self {
        return Self {
            matchOption: self.matchOption.to_owned()
        };
    }
}

impl<S: 'static + Debug> MatchOption<S> {
    pub fn new<T: 'static + Debug, F: 'static + MatchType<Value = T>, M: 'static + Clone + Fn(&ModuleRange, T) -> Result<S, ASTError>>(matchType: F, mappingFunction: M) -> Self {
        return Self {
            matchOption: Box::new(DynMatchOption {
                matchType,
                mappingFunction,
            })
        };
    }
}

pub fn getMatchAnyOf<S: 'static + Debug>(options: &[MatchOption<S>], conflictResolver: impl 'static + Clone + Fn(ModulePos, Vec<Match<S>>, Vec<ASTError>) -> Result<Match<S>, ASTError>) -> impl MatchType<Value = S> {
    struct MatchOptionType<S: 'static, R: 'static + Clone + Fn(ModulePos, Vec<Match<S>>, Vec<ASTError>) -> Result<Match<S>, ASTError>>(Vec<MatchOption<S>>, R);

    impl<S: 'static, R: 'static + Clone + Fn(ModulePos, Vec<Match<S>>, Vec<ASTError>) -> Result<Match<S>, ASTError>> Debug for MatchOptionType<S, R> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return writeln!(f, "getMatchAnyOf()");
        }
    }

    impl<S: 'static, R: 'static + Clone + Fn(ModulePos, Vec<Match<S>>, Vec<ASTError>) -> Result<Match<S>, ASTError>> Clone for MatchOptionType<S, R> {
        fn clone(&self) -> Self {
            return Self {
                0: self.0.to_owned(),
                1: self.1.to_owned(),
            };
        }
    }
    impl<S: 'static + Debug, R: 'static + Clone + Fn(ModulePos, Vec<Match<S>>, Vec<ASTError>) -> Result<Match<S>, ASTError>> MatchType for MatchOptionType<S, R> {
        type Value = S;

        fn getDescription(&self) -> String {
            return format!("Any({:?})", self.0.iter().map(|option| option.matchOption.getDescription()).collect::<Vec<String>>());
        }

        fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
            let mut matchVec = Vec::new();
            let mut errVec = Vec::new();
            for matchOption in &self.0 {
                match matchOption.matchOption.getMatchValue(startPos.to_owned()) {
                    Ok(matched) => matchVec.push(matched),
                    Err(err) => errVec.push(err),
                }
            }
            errVec.retain(|err| err.getModulePos() != &startPos);
            return (self.1)(startPos, matchVec, errVec);
        }
    }

    return MatchOptionType(options.to_vec(), conflictResolver);
}

pub fn getMatchOneOf<S: 'static + Debug>(options: &[MatchOption<S>]) -> impl MatchType<Value = S> {
    return getMatchAnyOf(options, |pos, mut options, err| {
        match options.len() {
            0 => Err(ASTError::MatchOptionsFailed(pos, err)),
            1 => Ok(options.pop().unwrap()),
            _ => Err(ASTError::MultipleConflict(pos, options.iter().map(|matchValue| format!("{matchValue:?}")).collect()))
        }
    });
}

pub fn getLazyMatch<S: Debug, M: MatchType<Value = S>>(f: impl 'static + Clone + Fn() -> M) -> impl MatchType<Value = S> {
    enum MatchLazy<S, M: MatchType<Value = S>, F: 'static + Clone + Fn() -> M> {
        Initialized(M),
        Uninitialized(F),
    }

    impl<S, M: MatchType<Value = S>, F: 'static + Clone + Fn() -> M> MatchLazy<S, M, F> {
        fn getInitialized(&mut self) -> &M {
            return match self {
                MatchLazy::Initialized(m) => m,
                MatchLazy::Uninitialized(f) => {
                    *self = MatchLazy::Initialized(f());

                    if let MatchLazy::Initialized(m) = self {
                        m
                    } else {
                        unreachable!()
                    }
                }
            };
        }
    }

    impl<S, M: MatchType<Value = S>, F: 'static + Clone + Fn() -> M> Debug for MatchLazy<S, M, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return writeln!(f, "getLazyMatch()");
        }
    }

    impl<S, M: MatchType<Value = S>, F: 'static + Clone + Fn() -> M> Clone for MatchLazy<S, M, F> {
        fn clone(&self) -> Self {
            return match self {
                MatchLazy::Initialized(v) => MatchLazy::Initialized(v.to_owned()),
                MatchLazy::Uninitialized(v) => MatchLazy::Uninitialized(v.to_owned()),
            };
        }
    }

    impl<S: Debug, M: MatchType<Value = S>, F: 'static + Clone + Fn() -> M> MatchType for RefCell<MatchLazy<S, M, F>> {
        type Value = S;

        fn getDescription(&self) -> String {
            return self.borrow_mut().getInitialized().getDescription();
        }

        fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
            return self.borrow_mut().getInitialized().getMatch(startPos);
        }
    }

    return RefCell::new(MatchLazy::Uninitialized(f));
}

pub fn getRepeatingMatch<S: Debug>(minimum: usize, matchValue: impl MatchType<Value = S>) -> impl MatchType<Value = Vec<S>> {
    #[derive(Debug)]
    struct MatchRepeat<T: MatchType<Value = S>, S>(usize, T);

    impl<T: MatchType<Value = S>, S> Clone for MatchRepeat<T, S> {
        fn clone(&self) -> Self {
            return Self {
                0: self.0,
                1: self.1.to_owned(),
            };
        }
    }

    impl<T: MatchType<Value = S>, S: Debug> MatchType for MatchRepeat<T, S> {
        type Value = Vec<S>;

        fn getDescription(&self) -> String {
            return format!("Repeat({}+, {:?})", self.0, self.1);
        }

        fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
            let mut matchVec = Vec::new();
            let endPos = startPos.getModule().getModulePos(startPos.getModule().getTokenVector().len());
            let mut pos = startPos.to_owned();
            let mut lastError = ASTError::ExpectedSymbol(startPos.to_owned()); // should only be returned when startPos == endPos and minimum > 0

            while pos != endPos {
                let nextMatch = self.1.getMatch(pos.to_owned());
                match nextMatch {
                    Ok(nextMatch) => {
                        let (range, value) = nextMatch.take();
                        pos = range.getEndPos();
                        matchVec.push(value);
                    }
                    Err(err) => {
                        lastError = err;
                        break;
                    }
                }
            }

            return if matchVec.len() < self.0 {
                Err(lastError)
            } else {
                Ok(Match::new(startPos.getModule().getModuleRange(startPos.getTokenIndex()..pos.getTokenIndex()), matchVec))
            };
        }
    }

    return MatchRepeat(minimum, matchValue);
}

pub trait TupleAppend {
    type Append<V: Debug>: Debug;

    fn append<V: Debug>(self, value: V) -> Self::Append<V>;
}

macro_rules! TupleMatchType {
    ($first:ident, $($names:ident),*) => {
        impl<$first: Debug, $($names: Debug),*> TupleAppend for ($first, $($names),*) {
            type Append<V: Debug> = (V, $first, $($names),*);

            fn append<V: Debug>(self, value: V) -> Self::Append<V> {
                let ($first, $($names),*) = self;
                return (value, $first, $($names),*);
            }
        }

        impl<$first: MatchType, $($names: MatchType),*,> MatchType for ($first, $($names),*,) where ($($names),*,): MatchType, <($($names),*, ) as MatchType>::Value: TupleAppend, <<($($names),*,) as MatchType>::Value as TupleAppend>::Append<<$first as MatchType>::Value>: TupleAppend {
            type Value = <<($($names),*,) as MatchType>::Value as TupleAppend>::Append<<$first as MatchType>::Value>;

            fn getDescription(&self) -> String {
                let ($first, $($names),*) = self;
                let tuple = ($first.getDescription(), $($names.getDescription()),*);
                return format!("{tuple:?}");
            }

            fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
                let ($first, $($names),*) = self;
                let firstMatch = $first.getMatch(startPos)?;
                let remainingMatch = ($($names.to_owned()),*,).getMatch(firstMatch.getRange().getEndPos())?;
                let (mut firstRange, firstValue) = firstMatch.take();
                let (range, value) = remainingMatch.take();

                firstRange.setEndIndex(range.getEndIndex());
                let value = value.append(firstValue);

                Ok(Match::new(firstRange, value))
            }
        }
    }
}

impl<T: Debug> TupleAppend for (T, ) {
    type Append<V: Debug> = (V, T);

    fn append<V: Debug>(self, value: V) -> Self::Append<V> {
        return (value, self.0);
    }
}

impl<T: MatchType> MatchType for (T, ) {
    type Value = (T::Value, );

    fn getDescription(&self) -> String {
        return self.0.getDescription();
    }

    fn getMatch(&self, startPos: ModulePos) -> Result<Match<Self::Value>, ASTError> {
        return self.0.getMatch(startPos).map(|value| {
            let (range, value) = value.take();
            return Match::new(range, (value, ));
        });
    }
}

TupleMatchType!(T0, T1);
TupleMatchType!(T0, T1, T2);
TupleMatchType!(T0, T1, T2, T3);
TupleMatchType!(T0, T1, T2, T3, T4);
TupleMatchType!(T0, T1, T2, T3, T4, T5);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
