use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use super::{
    geometric_traits::{
        EuclideanDistanceSquared, IterateNeighbours, ManhattanDistance, Movement4Directions,
    },
    linear_index::LinearIndex,
    math::AbsoluteValue,
};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<const C: usize, T> {
    pub values: [T; C],
}

impl<const C: usize, T: Display> Display for Vector<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector(")?;
        for x in 0..C {
            if x == C - 1 {
                write!(f, "{}", self.values[x])?;
            } else {
                write!(f, "{}, ", self.values[x])?;
            }
        }
        write!(f, ")")
    }
}

impl<const C: usize, T> Vector<C, T> {
    pub fn new(values: [T; C]) -> Self {
        Self { values }
    }
}

impl<const C: usize, T> Vector<C, T>
where
    T: Copy,
{
    pub fn all(value: T) -> Self {
        Self { values: [value; C] }
    }

    pub fn elementwise_unary<F: Fn(T) -> T>(&self, f: F) -> Self {
        let mut result = self.values;
        for x in 0..C {
            result[x] = f(result[x]);
        }
        Self::new(result)
    }
    pub fn aggregate_unary<F: Fn(T, T) -> T>(&self, f: F) -> T {
        let mut acc = self.values[0];
        for x in 1..C {
            acc = f(acc, self.values[x]);
        }
        acc
    }
    pub fn elementwise_binary<F: Fn(T, T) -> T>(&self, rhs: Self, f: F) -> Self {
        let mut result = self.values;
        for x in 0..C {
            result[x] = f(result[x], rhs.values[x]);
        }
        Self::new(result)
    }
}

impl<const C: usize, T> Vector<C, T>
where
    T: Copy,
    T: Ord,
{
    pub fn elementwise_min(&self, rhs: Self) -> Self {
        self.elementwise_binary(rhs, |a, b| a.min(b))
    }
    pub fn elementwise_max(&self, rhs: Self) -> Self {
        self.elementwise_binary(rhs, |a, b| a.min(b))
    }
}

impl<const C: usize, T> Vector<C, T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    pub fn inner(&self, rhs: Self) -> T {
        self.elementwise_binary(rhs, |a, b| a * b)
            .aggregate_unary(|acc, x| acc + x)
    }
}

impl V2i32 {
    pub fn winding(&self, rhs: Self) -> i32 {
        (self.x() * rhs.y()) - (self.y() * rhs.x())
    }
    pub fn perp(&self) -> Self {
        Self::from_xy(-self.y(), self.x())
    }
}

impl<const C: usize> Vector<C, f32> {
    pub fn magn(&self) -> f32 {
        self.inner(*self).sqrt()
    }
    pub fn normalized(&self) -> Self {
        let magn = self.magn();

        if magn > f32::EPSILON {
            *self * (1.0 / magn)
        } else {
            *self * 0.0
        }
    }
}

impl<const C: usize, T: Add<Output = T> + Copy> Add for Vector<C, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = self.values[x] + rhs.values[x];
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: AddAssign + Copy> AddAssign for Vector<C, T> {
    fn add_assign(&mut self, rhs: Self) {
        for x in 0..C {
            self.values[x] += rhs.values[x];
        }
    }
}
impl<const C: usize, T: Sub<Output = T> + Copy> Sub for Vector<C, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = self.values[x] - rhs.values[x];
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: SubAssign + Copy> SubAssign for Vector<C, T> {
    fn sub_assign(&mut self, rhs: Self) {
        for x in 0..C {
            self.values[x] -= rhs.values[x];
        }
    }
}

impl<const C: usize, T> Mul<T> for Vector<C, T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = values[x] * rhs;
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: MulAssign + Copy> MulAssign<T> for Vector<C, T> {
    fn mul_assign(&mut self, rhs: T) {
        for x in 0..C {
            self.values[x] *= rhs;
        }
    }
}

pub struct Scalar<T> {
    pub value: T,
}

impl<T> Scalar<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<const C: usize, T> Mul<Vector<C, T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<C, T>;

    fn mul(self, rhs: Vector<C, T>) -> Self::Output {
        rhs * self.value
    }
}

impl<const C: usize, T: FromStr + Debug> FromStr for Vector<C, T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_values = s
            .split(',')
            .map(|x| x.trim().parse::<T>())
            .collect::<Result<Vec<_>, _>>();

        if let Ok(vec_values) = vec_values {
            let values: [T; C] = vec_values.try_into().unwrap();
            Ok(Self::new(values))
        } else {
            Err(())
        }
    }
}

impl<const C: usize, T> ManhattanDistance<Vector<C, T>, T> for Vector<C, T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: AbsoluteValue,
{
    fn manhattan_distance(&self, other: &Self) -> T {
        let delta = *other - *self;
        let mut result = delta.values[0].abs().unwrap();
        for i in 1..C {
            result = result + delta.values[i].abs().unwrap();
        }
        result
    }
}

impl<const C: usize, T> EuclideanDistanceSquared<Vector<C, T>, T> for Vector<C, T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    fn euclidean_distance_squared(&self, other: &Self) -> T {
        let delta = *other - *self;
        delta.inner(delta)
    }
}

macro_rules! vector_from {
    ($t:ty; $($u:ty),*) => {$(
        impl<const C: usize> From<Vector<C, $u>> for Vector<C, $t> {
            fn from(value: Vector<C, $u>) -> Self {
                let mut values: [$t; C] = [0 as $t; C];

                for x in 0..C {
                    values[x] = value.values[x].into();
                }

                Self::new(values)
            }
        })*
    };
}
macro_rules! vector_try_from {
    ($t:ty; $($u:ty),*) => {$(
        impl<const C: usize> TryFrom<Vector<C, $u>> for Vector<C, $t> {
            type Error = <$t as std::convert::TryFrom<$u>>::Error;

            fn try_from(value: Vector<C, $u>) -> Result<Self, Self::Error> {
                let mut values: [$t; C] = [0 as $t; C];

                for x in 0..C {
                    values[x] = value.values[x].try_into()?;
                }

                Ok(Self::new(values))
            }
        })*
    };
}

vector_from!(i128; i64, i32, i16, i8);
vector_from!(i64; i32, i16, i8);
vector_from!(i32; i16, i8);
vector_from!(i16; i8);
vector_from!(u128; u64, u32, u16, u8);
vector_from!(u64; u32, u16, u8);
vector_from!(u32; u16, u8);
vector_from!(u16; u8);
vector_from!(f64; f32);

vector_try_from!(usize; i128, i64, i32, i16, i8, u128, u64, u32, u16, u8, isize);
vector_try_from!(i32; i128, i64, u128, u64, u32, u16, u8, isize, usize);

macro_rules! vector_linear_index {
    ($($t:ty),*) => {
        $(
impl<const N: usize> LinearIndex<Self> for Vector<N, $t> {
    fn index_unchecked(&self, i: Self) -> Option<usize> {
        let mut result: usize = 0;
        for j in (0..N).rev() {
            let a: usize = self.values[j].try_into().unwrap();
            result *= a;
            let b: usize = i.values[j].try_into().unwrap();
            result += b;
        }
        Some(result)
    }

    fn unindex(&self, mut i: usize) -> Option<Self> {
        let mut result = Vector::new([0; N]);
        for j in 0..N {
            result.values[j] = (i % self.values[j] as usize).try_into().unwrap();
            i /= self.values[j] as usize;
        }
        Some(result)
    }

    #[allow(unused_comparisons)]
    fn is_in_bounds(&self, i: &Self) -> bool {
        i.values
            .iter()
            .zip(self.values)
            .all(|(&a, b)| a >= 0 && a < b)
    }
}
        )*
    };
}

vector_linear_index!(usize, isize, i128, i64, i32, i16, i8, u128, u64, u32, u16, u8);

macro_rules! movement4directions {
    ($($t:ty),*) => {
        $(
            impl<const C: usize> IterateNeighbours<()> for Vector<C, $t> {
                fn neighbours(&self, _context: &()) -> Vec<Self> {
                    let mut results = vec![];

                    for i in 0..C {
                        if let Some(a) = self.values[i].checked_add(1) {
                            let mut b = self.clone();
                            b.values[i] = a;
                            results.push(b);
                        }
                        if let Some(a) = self.values[i].checked_sub(1) {
                            let mut b = self.clone();
                            b.values[i] = a;
                            results.push(b);
                        }
                    }

                    results
                }
            }
        )*
    };
}

movement4directions!(i32, usize);

pub type V2<T> = Vector<2, T>;
pub type V3<T> = Vector<3, T>;
pub type V4<T> = Vector<4, T>;

pub type V2i32 = V2<i32>;
pub type V2usize = V2<usize>;
// pub type V2f32 = V2<f32>;
// pub type V3i32 = V3<i32>;
// pub type V3usize = V3<usize>;
// pub type V3f32 = V3<f32>;
// pub type V4i32 = V4<i32>;
// pub type V4usize = V4<usize>;
// pub type V4f32 = V4<f32>;

impl<T: Copy> V2<T> {
    pub fn from_xy(x: T, y: T) -> Self {
        Self { values: [x, y] }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
}

macro_rules! movement4directions {
    ($v:ident; $($t:ty),*) => {
        $(
        impl Movement4Directions for $v<$t> {
            fn step_right(&self) -> Option<Self> {
                if let Some(x) = self.x().checked_add(1) {
                    Some(V2::from_xy(x, self.y()))
                } else {
                    None
                }
            }
            fn step_up(&self) -> Option<Self> {
                if let Some(y) = self.y().checked_add(1) {
                    Some(V2::from_xy(self.x(), y))
                } else {
                    None
                }
            }
            fn step_left(&self) -> Option<Self> {
                if let Some(x) = self.x().checked_sub(1) {
                    Some(V2::from_xy(x, self.y()))
                } else {
                    None
                }
            }
            fn step_down(&self) -> Option<Self> {
                if let Some(y) = self.y().checked_sub(1) {
                    Some(V2::from_xy(self.x(), y))
                } else {
                    None
                }
            }
        })*
    };
}

movement4directions!(V2; usize, i32);

impl<T: Copy> V3<T> {
    pub fn from_xyz(x: T, y: T, z: T) -> Self {
        Self { values: [x, y, z] }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
    pub fn z(&self) -> T {
        self.values[2]
    }
}

impl<T: Copy> V4<T> {
    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self {
            values: [x, y, z, w],
        }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
    pub fn z(&self) -> T {
        self.values[2]
    }
    pub fn w(&self) -> T {
        self.values[3]
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::V3;

    #[test]
    fn v3_eq() {
        let a = V3::from_xyz(0, 0, 0);
        let b = V3::from_xyz(0, 0, 0);
        assert!(a == b);
        assert_eq!(a, b);
    }
}
