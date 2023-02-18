use crate::module::modulepos::{ModulePos, ModuleRange};

pub trait MatchType: Clone {
    type Value;

    fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>>;
}

pub struct Match<T> {
    range: ModuleRange,
    value: T,
}

impl<T> Match<T> {
    pub fn new(range: ModuleRange, value: T) -> Self {
        return Self {
            range,
            value
        };
    }

    pub fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }

    pub fn take(self) -> (ModuleRange, T) {
        return (self.range, self.value);
    }
}

#[derive(Clone)]
pub struct OptionalMatch<T: MatchType>(T);

impl<T: MatchType> OptionalMatch<T> {
    pub fn new(value: T) -> Self {
        return Self {
            0: value,
        };
    }
}

impl<T: MatchType> MatchType for OptionalMatch<T> {
    type Value = Option<T::Value>;

    fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>> {
        return Some(if let Some(matched) = self.0.getMatch(startPos.to_owned()) {
            let (range, value) = matched.take();
            Match::new(range, Some(value))
        } else {
            Match::new(startPos.getRange(0), None)
        });
    }
}

pub fn getMatchFrom<S>(function: impl 'static + Clone + Fn(ModulePos) -> Option<Match<S>>) -> impl MatchType<Value = S> {
    struct MatchImpl<F: 'static + Clone + Fn(ModulePos) -> Option<Match<S>>, S> {
        function: F
    }

    impl<F: 'static + Clone + Fn(ModulePos) -> Option<Match<S>>, S> Clone for MatchImpl<F, S> {
        fn clone(&self) -> Self {
            return Self {
                function: self.function.to_owned(),
            }
        }
    }
    
    impl<F: 'static + Clone + Fn(ModulePos) -> Option<Match<S>>, S> MatchType for MatchImpl<F, S> {
        type Value = S;

        fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>> {
            return (self.function)(startPos);
        }
    }

    return MatchImpl {
        function
    };
}

pub fn getNestedMatch<S, T: MatchType>(matcher: T, function: impl 'static + Clone + Fn(ModuleRange, T::Value) -> Option<S>) -> impl MatchType<Value = S> {
    struct MatchNested<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Option<S>> {
        matcher: T,
        function: F
    }

    impl<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Option<S>> Clone for MatchNested<S, T, F> {
        fn clone(&self) -> Self {
            return Self {
                matcher: self.matcher.to_owned(),
                function: self.function.to_owned(),
            }
        }
    }

    impl<S, T: MatchType, F: 'static + Clone + Fn(ModuleRange, T::Value) -> Option<S>> MatchType for MatchNested<S, T, F> {
        type Value = S;

        fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>> {
            return if let Some(matched) = self.matcher.getMatch(startPos) {
                let (range, value) = matched.take();
                Some(Match::new(range.to_owned(), (self.function)(range, value)?))
            } else {
                None
            }
        }
    }

    return MatchNested {
        matcher,
        function
    }
}

pub trait TupleAppend {
    type Append<V>;

    fn append<V>(self, value: V) -> Self::Append<V>;
}

macro_rules! TupleMatchType {
    ($first:ident, $($names:ident),*) => {
        impl<$first, $($names),*> TupleAppend for ($first, $($names),*) {
            type Append<V> = (V, $first, $($names),*);

            fn append<V>(self, value: V) -> Self::Append<V> {
                let ($first, $($names),*) = self;
                return (value, $first, $($names),*);
            }
        }

        impl< $first: MatchType, $($names: MatchType),*,> MatchType for ($first, $($names),*,) where ($($names),*,): MatchType, <($($names),*, ) as MatchType>::Value: TupleAppend, <<($($names),*,) as MatchType>::Value as TupleAppend>::Append<<$first as MatchType>::Value>: TupleAppend {
            type Value = <<($($names),*,) as MatchType>::Value as TupleAppend>::Append<<$first as MatchType>::Value>;

            fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>> {
                let ($first, $($names),*) = self;
                return $first.getMatch(startPos).map(|firstMatch| {
                    if let Some(remainingMatch) = ($($names.to_owned()),*,).getMatch(firstMatch.getRange().getEndPos()) {
                        let (mut firstRange, firstValue) = firstMatch.take();
                        let (range, value) = remainingMatch.take();

                        firstRange.setEndIndex(range.getEndIndex());
                        let value = value.append(firstValue);

                        Some(Match::new(firstRange, value))
                    } else {
                        None
                    }
                }).flatten();
            }
        }
    }
}

impl<T> TupleAppend for (T,) {
    type Append<V> = (V, T);

    fn append<V>(self, value: V) -> Self::Append<V> {
        return (value, self.0);
    }
}

impl<T: MatchType> MatchType for (T,) {
    type Value = (T::Value,);

    fn getMatch(&self, startPos: ModulePos) -> Option<Match<Self::Value>> {
        return self.0.getMatch(startPos).map(|value| {
            let (range, value) = value.take();
            return Match::new(range, (value,));
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
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
TupleMatchType!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
