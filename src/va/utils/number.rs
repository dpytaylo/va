use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait Number:
    Add
    + AddAssign
    + Copy
    + Debug
    + Default
    + Display
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + PartialEq
    + PartialOrd
    + Rem
    + RemAssign
    + Sub
    + SubAssign
    + Sum
    + Sum<<Self as Mul>::Output>
    + Sum<<Self as Div>::Output>
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    const MIN: Self;
    const MAX: Self;

    fn abs(self) -> Self;
    fn sqr(self) -> Self;
}

macro_rules! impl_number_cast {
    ($a:ty, $($n:ty),*) => {
        $(
            impl crate::utils::cast::Cast<$n> for $a {
                fn cast(self) -> $n {
                    self as $n
                }
            }
        )*
    };
}

macro_rules! impl_number_methods {
    ($($n:ty),*) => {
        $(
            impl Number for $n {
                const ZERO: Self = 0 as $n;
                const ONE: Self = 1 as $n;
                const TWO: Self = 2 as $n;

                const MIN: Self = <$n>::MIN;
                const MAX: Self = <$n>::MAX;

                #[allow(unconditional_recursion)]
                fn abs(self) -> Self {
                    self.abs()
                }

                fn sqr(self) -> Self {
                    self.powi(2)
                }
            }

            impl_number_cast!($n, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
        )*
    };
}

impl_number_methods!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait Integer {
    fn powi(self, n: u32) -> Self;
}

macro_rules! impl_integer_methods {
    ($($n:ty),*) => {
        $(
            impl Integer for $n {
                fn powi(self, n: u32) -> Self {
                    self.pow(n)
                }
            }
        )*
    };
}

impl_integer_methods!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait Float {
    fn new(value: f64) -> Self;
    fn ceil(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn floor(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn round(self) -> Self;
    fn sqrt(self) -> Self;
}

macro_rules! impl_float_methods {
    ($($n:ty),*) => {
        $(
            impl Float for $n {
                fn new(value: f64) -> Self {
                    value as $n
                }

                fn ceil(self) -> Self {
                    self.ceil()
                }

                fn clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }

                fn floor(self) -> Self {
                    self.floor()
                }

                fn powi(self, n: i32) -> Self {
                    self.powi(n)
                }

                fn powf(self, n: Self) -> Self {
                    self.powf(n)
                }

                fn round(self) -> Self {
                    self.round()
                }

                fn sqrt(self) -> Self {
                    self.sqrt()
                }
            }
        )*
    };
}

impl_float_methods!(f32, f64);

pub trait Signed {}

macro_rules! impl_signed_methods {
    ($($n:ty),*) => {
        $(
            impl Signed for $n {}
        )*
    };
}

impl_signed_methods!(i8, i16, i32, i64, i128, f32, f64, isize);

pub trait Pow<T = Self> {
    fn pow(self, n: T) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_two_numbers<T>(a: T, b: T) -> T 
        where T: Number + std::ops::Add<Output = T>,
    {
        a + b
    }

    #[test]
    fn number_test() {
        assert_eq!(add_two_numbers(10, 20), 30);
    }
}