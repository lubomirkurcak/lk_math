pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! zero {
    ($($t:ty),*) => {
        $(
        impl Zero for $t {
    fn zero() -> Self {
        0 as $t
    }
        })*
    };
}

macro_rules! one {
    ($($t:ty),*) => {
        $(
        impl One for $t {
    fn one() -> Self {
        1 as $t
    }
        })*
    };
}

zero!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128, f32, f64);
one!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128, f32, f64);

pub trait Gcd {
    fn gcd(a: Self, b: Self) -> Self;
    fn lcm(a: Self, b: Self) -> Self;
}

// NOTE(lubo): adapted https://rosettacode.org/wiki/Least_common_multiple#Rust
macro_rules! gcd {
    ($($t:ty),*) => {
        $(
        impl Gcd for $t {
            fn gcd(a: Self, b: Self) -> Self {
                match ((a, b), (a & 1, b & 1)) {
                    ((x, y), _) if x == y => y,
                    ((0, x), _) | ((x, 0), _) => x,
                    ((x, y), (0, 1)) | ((y, x), (1, 0)) => Self::gcd(x >> 1, y),
                    ((x, y), (0, 0)) => Self::gcd(x >> 1, y >> 1) << 1,
                    ((x, y), (1, 1)) => {
                        let (x, y) = (std::cmp::min(x, y), std::cmp::max(x, y));
                        Self::gcd((y - x) >> 1, x)
                    }
                    _ => unreachable!(),
                }
            }
            fn lcm(a: Self, b: Self) -> Self {
                a * (b / Self::gcd(a, b))
            }
        })*
    };
}

gcd!(usize, i32, i64);

pub trait AbsoluteValue
where
    Self: Sized,
{
    fn abs(&self) -> Option<Self>;
}

macro_rules! checked_absolute_value {
    ($($t:ty),*) => {
        $(
        impl AbsoluteValue for $t {
    fn abs(&self) -> Option<Self> {
        self.checked_abs()
    }
        })*
    };
}

checked_absolute_value!(i32);

macro_rules! identity_absolute_value {
    ($($t:ty),*) => {
        $(
impl AbsoluteValue for $t {
    fn abs(&self) -> Option<Self> {
        Some(*self)
    }
        })*
    };
}

identity_absolute_value!(usize);

pub trait InclusiveMin<T> {
    fn inclusive_min(&self) -> &T;
}
pub trait InclusiveMax<T> {
    fn inclusive_max(&self) -> &T;
}
pub trait ExclusiveMax<T> {
    fn exclusive_max(&self) -> &T;
}

pub fn triangle_numbers(n: i32) -> i32 {
    // n * (n + 1) / 2
    if n & 0b1 > 0 {
        n * ((n + 1) / 2)
    } else {
        (n / 2) * (n + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_numbers() {
        assert_eq!(triangle_numbers(1), 1);
        assert_eq!(triangle_numbers(2), 3);
        assert_eq!(triangle_numbers(3), 6);
        assert_eq!(triangle_numbers(4), 10);
        assert_eq!(triangle_numbers(5), 15);
        assert_eq!(triangle_numbers(6), 21);
        assert_eq!(triangle_numbers(7), 28);
        assert_eq!(triangle_numbers(8), 36);
        assert_eq!(triangle_numbers(9), 45);
        assert_eq!(triangle_numbers(10), 55);
    }
}
