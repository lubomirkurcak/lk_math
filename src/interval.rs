use std::ops::{Mul, Sub};

pub trait InclusiveMin<T> {
    fn inclusive_min(&self) -> &T;
}
// pub trait InclusiveMax<T> {
//     fn inclusive_max(&self) -> &T;
// }
pub trait ExclusiveMax<T> {
    fn exclusive_max(&self) -> &T;
}

pub trait Halfopen<T> {
    fn halfopen_bounds(&self) -> (&T, &T);
}
impl<A, T> Halfopen<T> for A
where
    A: InclusiveMin<T>,
    A: ExclusiveMax<T>,
{
    fn halfopen_bounds(&self) -> (&T, &T) {
        (self.inclusive_min(), self.exclusive_max())
    }
}

pub trait Interval
where
    Self: Sized,
{
    fn intersection(&self, other: &Self) -> Option<Self>;
    fn union(&self, other: &Self) -> Option<Self>;
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
    fn dominates(&self, other: &Self) -> bool;
}

// impl<T> InclusiveIntervalOverlap for T
// where
//     T: PartialOrd,
//     Self: InclusiveMin<T> + InclusiveMax<T>,
// {
//     fn inclusive_interval_overlap_test(&self, other: &Self) -> bool {
//         // a1 >= b0 && a0 <= b1
//         self.inclusive_max() >= other.inclusive_min()
//             && self.inclusive_min() <= other.inclusive_max()
//     }
//
//     fn inclusive_interval_union(&self, other: &Self) -> Self {
//         todo!()
//     }
// }

impl<T> InclusiveMin<T> for std::ops::Range<T> {
    fn inclusive_min(&self) -> &T {
        &self.start
    }
}
impl<T> ExclusiveMax<T> for std::ops::Range<T> {
    fn exclusive_max(&self) -> &T {
        &self.end
    }
}
// impl<T> InclusiveMax<T> for std::ops::Range<T>
// where T:crate::math::One, T:Sub<Output = T> {
//     fn inclusive_max(&self) -> &T {
//         &(self.end - T::one())
//     }
// }
// impl<T> InclusiveMin<T> for std::ops::RangeInclusive<T> {
//     fn inclusive_min(&self) -> &T {
//         &self.start()
//     }
// }
// impl<T> InclusiveMax<T> for std::ops::RangeInclusive<T> {
//     fn inclusive_max(&self) -> &T {
//         &self.end()
//     }
// }

impl<T> Interval for std::ops::Range<T>
where
    T: Copy + Ord,
{
    fn intersection(&self, other: &Self) -> Option<Self> {
        let (mut a0, mut a1) = self.halfopen_bounds();
        let (mut b0, mut b1) = other.halfopen_bounds();

        if a0 > b0 {
            std::mem::swap(&mut a0, &mut b0);
            std::mem::swap(&mut a1, &mut b1);
        }

        // NOTE(lubo): Current policy is to return None
        // 1. use `a1 < b0` to get `Some(a..a)`
        // 2. use `a1 <= b0` to get `None`
        if a1 <= b0 {
            None
        } else {
            Some(*b0..*std::cmp::min(a1, b1))
        }
    }

    fn union(&self, other: &Self) -> Option<Self> {
        let (mut a0, mut a1) = self.halfopen_bounds();
        let (mut b0, mut b1) = other.halfopen_bounds();

        if a0 > b0 {
            std::mem::swap(&mut a0, &mut b0);
            std::mem::swap(&mut a1, &mut b1);
        }

        if a1 < b0 {
            None
        } else {
            Some(*a0..*std::cmp::max(a1, b1))
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        let (a0, a1) = self.halfopen_bounds();
        let (b0, b1) = other.halfopen_bounds();
        a1 > b0 && a0 < b1
    }

    fn touches(&self, other: &Self) -> bool {
        let (a0, a1) = self.halfopen_bounds();
        let (b0, b1) = other.halfopen_bounds();
        a1 >= b0 && a0 <= b1
    }

    fn dominates(&self, other: &Self) -> bool {
        let (&a0, &a1) = self.halfopen_bounds();
        let (&b0, &b1) = other.halfopen_bounds();
        a0 <= b0 && a1 >= b1
    }
}

pub trait UniversalInterval: Sized {
    const INFINUM: Self;
    const SUPREMUM: Self;

    fn is_infinum(&self) -> bool;
    fn is_supremum(&self) -> bool;

    fn universal_interval() -> std::ops::Range<Self> {
        Self::INFINUM..Self::SUPREMUM
    }
}

macro_rules! impl_universal_interval {
    ($inf:ident, $sup:ident; $($t:ty),*) => {
        $(
            impl UniversalInterval for $t {
                const INFINUM: Self = <$t>::$inf;
                const SUPREMUM: Self = <$t>::$sup;

                fn is_infinum(&self) -> bool {
                    *self == Self::INFINUM
                }

                fn is_supremum(&self) -> bool {
                    *self == Self::SUPREMUM
                }
            }
        )*
    };
}

impl_universal_interval!(MIN, MAX; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_universal_interval!(NEG_INFINITY, INFINITY; f32, f64, crate::ord_float::OrdF32, crate::ord_float::OrdF64);

pub trait IntervalExt
where
    Self: Sized,
{
    fn dominates_or_is_dominated_by(&self, other: &Self) -> bool;
}

impl<T> IntervalExt for std::ops::Range<T>
where
    T: Copy + Ord,
    T: crate::math::Zero + Sub<Output = T> + Mul<Output = T>,
{
    fn dominates_or_is_dominated_by(&self, other: &Self) -> bool {
        let (&a0, &a1) = self.halfopen_bounds();
        let (&b0, &b1) = other.halfopen_bounds();
        (b0 - a0) * (b1 - a1) <= T::zero()
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::{Interval, IntervalExt};

    #[test]
    fn abab() {
        let a = 0..2;
        let b = 1..3;
        assert_eq!(a.union(&b), Some(0..3));
        assert_eq!(b.union(&a), Some(0..3));
        assert_eq!(a.intersection(&b), Some(1..2));
        assert_eq!(b.intersection(&a), Some(1..2));
        assert!(a.overlaps(&b));
        assert!(a.touches(&b));
        assert!(!a.dominates(&b));
        assert!(!a.dominates_or_is_dominated_by(&b));
        assert!(b.overlaps(&a));
        assert!(b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(!b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
    #[test]
    fn abba() {
        let a = 0..3;
        let b = 1..2;
        assert_eq!(a.union(&b), Some(0..3));
        assert_eq!(b.union(&a), Some(0..3));
        assert_eq!(a.intersection(&b), Some(1..2));
        assert_eq!(b.intersection(&a), Some(1..2));
        assert!(a.overlaps(&b));
        assert!(a.touches(&b));
        assert!(a.dominates(&b));
        assert!(a.dominates_or_is_dominated_by(&b));
        assert!(b.overlaps(&a));
        assert!(b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
    #[test]
    fn aabb() {
        let a = 0..1;
        let b = 2..3;
        assert_eq!(a.union(&b), None);
        assert_eq!(b.union(&a), None);
        assert_eq!(a.intersection(&b), None);
        assert_eq!(b.intersection(&a), None);
        assert!(!a.overlaps(&b));
        assert!(!a.touches(&b));
        assert!(!a.dominates(&b));
        assert!(!a.dominates_or_is_dominated_by(&b));
        assert!(!b.overlaps(&a));
        assert!(!b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(!b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
    #[test]
    fn abx() {
        let a = 0..2;
        let b = 1..2;
        assert_eq!(a.union(&b), Some(0..2));
        assert_eq!(b.union(&a), Some(0..2));
        assert_eq!(a.intersection(&b), Some(1..2));
        assert_eq!(b.intersection(&a), Some(1..2));
        assert!(a.overlaps(&b));
        assert!(a.touches(&b));
        assert!(a.dominates(&b));
        assert!(a.dominates_or_is_dominated_by(&b));
        assert!(b.overlaps(&a));
        assert!(b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
    #[test]
    fn xab() {
        let a = 0..3;
        let b = 0..2;
        assert_eq!(a.union(&b), Some(0..3));
        assert_eq!(b.union(&a), Some(0..3));
        assert_eq!(a.intersection(&b), Some(0..2));
        assert_eq!(b.intersection(&a), Some(0..2));
        assert!(a.overlaps(&b));
        assert!(a.touches(&b));
        assert!(a.dominates(&b));
        assert!(a.dominates_or_is_dominated_by(&b));
        assert!(b.overlaps(&a));
        assert!(b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
    #[test]
    fn axb() {
        let a = 0..2;
        let b = 2..3;
        assert_eq!(a.union(&b), Some(0..3));
        assert_eq!(b.union(&a), Some(0..3));

        assert_eq!(a.intersection(&b), None);
        assert_eq!(b.intersection(&a), None);
        // assert_eq!(a.interval_intersection(&b), Some(2..2));
        // assert_eq!(b.interval_intersection(&a), Some(2..2));
        // assert!(a.interval_intersection(&b).is_none() || a.interval_intersection(&b) == Some(2..2));
        // assert!(b.interval_intersection(&a).is_none() || b.interval_intersection(&a) == Some(2..2));

        assert!(!a.overlaps(&b));
        assert!(a.touches(&b));
        assert!(!a.dominates(&b));
        assert!(!a.dominates_or_is_dominated_by(&b));
        assert!(!b.overlaps(&a));
        assert!(b.touches(&a));
        assert!(!b.dominates(&a));
        assert!(!b.dominates_or_is_dominated_by(&a));

        assert_eq!(a.overlaps(&b), a.intersection(&b).is_some());
        assert_eq!(b.overlaps(&a), b.intersection(&a).is_some());
    }
}
