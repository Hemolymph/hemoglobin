use super::{Compare, Comparison, MaybeImprecise};
use std::cmp::max;
use std::cmp::min;
use std::ops::Not;

/// Represents whether a query has been matched or not. This is not always a boolean value, but instead a ternary value, as cards may have undefined properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ternary {
    /// Card did not have the requested property.
    Void,
    /// Card had the requested property and it did not matched the requested value.
    False,
    /// Card had the requested property and it matched the requested value.
    True,
}

impl From<Ternary> for bool {
    fn from(value: Ternary) -> Self {
        matches!(value, Ternary::True)
    }
}

impl Ternary {
    /// A ternary OR which outputs the highest-valued result between `self` and `b`, where a `Match` is considered highest and `NotHave` is considered lowest.
    #[must_use]
    pub fn is_true(self) -> bool {
        self == Self::True
    }
    #[must_use]
    pub fn is_false(self) -> bool {
        self == Self::False
    }
    #[must_use]
    pub fn is_void(self) -> bool {
        self == Self::Void
    }
    /// A ternary OR which outputs the highest-valued result between `self` and `b`, where a `Match` is considered highest and `NotHave` is considered lowest.
    #[must_use]
    pub fn or(self, b: Self) -> Self {
        max(self, b)
    }
    /// A ternary XOR which outputs the highest-valued result between `self` and `b`, if they are not equal.
    /// If both values are `Match` or `NotMatch`, the output will be `NotMatch`.
    /// If both values are `NotHave`, the output will be `NotHave`.
    /// If no value is Match and there is a `NotHave`, the output will be `NotHave`.
    #[must_use]
    pub const fn xor(self, b: Self) -> Self {
        match (self, b) {
            (Self::Void, Self::Void) => Self::Void,
            (Self::True, Self::False | Self::Void) | (Self::False | Self::Void, Self::True) => {
                Self::True
            }
            (Self::False | Self::Void, Self::False)
            | (Self::False, Self::Void)
            | (Self::True, Self::True) => Self::False,
        }
    }
    /// A ternary AND which outputs the lowest-valued result between `self` and `b`, where a `Match` is considered highest and `NotHave` is considered lowest.
    #[must_use]
    pub fn and(self, b: Self) -> Self {
        min(self, b)
    }
}

impl Not for Ternary {
    type Output = Self;

    /// Ternary NOT where `NotHave` is considered opposite to itself.
    fn not(self) -> Self::Output {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Void => Self::Void,
        }
    }
}

impl From<bool> for Ternary {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl<T: Compare> Compare for Option<T> {
    fn gt(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.gt(comparison))
    }

    fn gt_eq(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.gt_eq(comparison))
    }

    fn lt(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.lt(comparison))
    }

    fn lt_eq(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.lt_eq(comparison))
    }

    fn eq(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.eq(comparison))
    }

    fn ne(&self, comparison: usize) -> Ternary {
        self.as_ref().map_or(Ternary::Void, |x| x.ne(comparison))
    }
}

impl Compare for MaybeImprecise {
    fn gt(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (x.assume() > comparison).into(),
            Self::Imprecise(x) => match x {
                Comparison::GreaterThan(_)
                | Comparison::GreaterThanOrEqual(_)
                | Comparison::NotEqual(_) => Ternary::True,
                Comparison::LowerThan(x) => (*x > comparison + 1).into(),
                Comparison::LowerThanOrEqual(x) | Comparison::Equal(x) => (*x > comparison).into(),
            },
        }
    }

    fn gt_eq(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (x.assume() >= comparison).into(),
            Self::Imprecise(x) => match x {
                Comparison::Equal(x) => (*x >= comparison).into(),
                Comparison::GreaterThan(_)
                | Comparison::GreaterThanOrEqual(_)
                | Comparison::NotEqual(_) => Ternary::True,
                Comparison::LowerThan(x) | Comparison::LowerThanOrEqual(x) => {
                    (*x > comparison).into()
                }
            },
        }
    }

    fn lt(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (x.assume() < comparison).into(),
            Self::Imprecise(x) => match x {
                Comparison::GreaterThan(x) => (*x < comparison - 1).into(),
                Comparison::GreaterThanOrEqual(x) | Comparison::Equal(x) => {
                    (*x < comparison).into()
                }
                Comparison::LowerThan(_)
                | Comparison::LowerThanOrEqual(_)
                | Comparison::NotEqual(_) => Ternary::True,
            },
        }
    }

    fn lt_eq(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (x.assume() <= comparison).into(),
            Self::Imprecise(x) => match x {
                Comparison::Equal(x) => (*x <= comparison).into(),
                Comparison::GreaterThan(x) | Comparison::GreaterThanOrEqual(x) => {
                    (*x < comparison).into()
                }
                Comparison::LowerThan(_)
                | Comparison::LowerThanOrEqual(_)
                | Comparison::NotEqual(_) => Ternary::True,
            },
        }
    }

    fn eq(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (comparison == x.assume()).into(),
            Self::Imprecise(x) => match x {
                Comparison::Equal(x) => (comparison == *x).into(),
                Comparison::GreaterThan(x) => (comparison > *x).into(),
                Comparison::GreaterThanOrEqual(x) => (comparison >= *x).into(),
                Comparison::LowerThan(x) => (comparison < *x).into(),
                Comparison::LowerThanOrEqual(x) => (comparison <= *x).into(),
                Comparison::NotEqual(x) => (comparison != *x).into(),
            },
        }
    }

    fn ne(&self, comparison: usize) -> Ternary {
        match self {
            Self::Precise(x) => (comparison != x.assume()).into(),
            Self::Imprecise(x) => match x {
                Comparison::Equal(x) => (comparison != *x).into(),
                Comparison::GreaterThan(_)
                | Comparison::GreaterThanOrEqual(_)
                | Comparison::LowerThan(_)
                | Comparison::LowerThanOrEqual(_)
                | Comparison::NotEqual(_) => Ternary::True,
            },
        }
    }
}

impl Compare for usize {
    fn gt(&self, comparison: usize) -> Ternary {
        (*self > comparison).into()
    }

    fn gt_eq(&self, comparison: usize) -> Ternary {
        (*self >= comparison).into()
    }

    fn lt(&self, comparison: usize) -> Ternary {
        (*self < comparison).into()
    }

    fn lt_eq(&self, comparison: usize) -> Ternary {
        (*self <= comparison).into()
    }

    fn eq(&self, comparison: usize) -> Ternary {
        (*self == comparison).into()
    }

    fn ne(&self, comparison: usize) -> Ternary {
        (*self != comparison).into()
    }
}
