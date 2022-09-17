use std::fmt::Display;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

use crate::va::utils::math::is_equal::IsCopyTypeEqual;
use crate::va::utils::cast::Cast;
use crate::va::utils::number::{Number, Float, Integer, Pow};

use super::utils::*;

const LENGTH: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vec2<T> 
    where T: Number,
{
    pub x: T,
    pub y: T,
}

impl_common_vector_methods!(Vec2);

impl<T> Vec2<T>
    where T: Number,
          Vec2<T>: FromIterator<T>,
{
    pub const ZERO: Vec2<T> = Vec2::new(T::ZERO, T::ZERO);
    pub const ONE: Vec2<T> = Vec2::new(T::ONE, T::ONE);
    pub const TWO: Vec2<T> = Vec2::new(T::TWO, T::TWO);

    pub const X: Vec2<T> = Vec2::new(T::ONE, T::ZERO);
    pub const Y: Vec2<T> = Vec2::new(T::ZERO, T::ONE);

    pub const MIN: Vec2<T> = Vec2::new(T::MIN, T::MIN);
    pub const MAX: Vec2<T> = Vec2::new(T::MAX, T::MAX);

    pub const fn new(x: T, y: T) -> Self {
        Self {
            x, y
        }
    }
}

impl<T> From<T> for Vec2<T>
    where T: Number,
{
    fn from(value: T) -> Self {
        Self::new(value, value)
    }
}

impl<T> From<(T, T)> for Vec2<T> 
    where T: Number,
{
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<Vec2<T>> for (T, T) 
    where T: Number,
{
    fn from(value: Vec2<T>) -> Self {
        (value[0], value[1])
    }
}

impl<T> FromIterator<T> for Vec2<T>
    where T: Number,
{
    /// # Panics
    /// 
    /// Panics if iterator is invalid
    fn from_iter<Iter>(iter: Iter) -> Self 
        where Iter: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        Self {
            x: iter.next().expect("invalid iterator"),
            y: iter.next().expect("invalid iterator"),
        } 
    }
}

impl<T> Index<usize> for Vec2<T>
    where T: Number,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T>
    where T: Number,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> Display for Vec2<T>
    where T: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}; {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::cast::Cast;

    use super::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn vector2d_comparison() {
        let a = Vec2 {
            x: 20,
            y: 54,
        };

        let b = Vec2 {
            x: 20,
            y: 54,
        };

        let c = Vec2 {
            x: 334,
            y: 54,
        };

        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_ne!(a, c);

        let a = Vec2 {
            x: 20.0,
            y: 54.0,
        };

        let b = Vec2 {
            x: 20.0,
            y: 54.0,
        };

        let c = Vec2 {
            x: 334.0,
            y: 54.0,
        };

        assert!(a.is_equal(b, EPSILON));
        assert!(!a.is_equal(c, EPSILON));

        assert!(a < c);
    }

    #[test]
    fn vector2d_initialization() {
        let vector = Vec2::new(1, 2);
        assert_eq!(vector, Vec2 {x: 1, y: 2});

        let tmp = Vec2::from(1);
        assert_eq!(tmp, Vec2::new(1, 1));

        let tmp: Vec2<i32> = Vec2::from(Vec2::new(1i8, 2));
        assert_eq!(tmp, vector);

        let tmp: Vec2<i32> = Vec2::try_from(Vec2::new(1i64, 2)).unwrap();
        assert_eq!(tmp, vector);

        let tmp: Result<Vec2<u8>, ()> = Vec2::try_from(Vec2::new(-1i32, 2));
        assert!(tmp.is_err());

        let tmp = Vec2::from((1, 2));
        assert_eq!(tmp, vector);

        let (x, y) = tmp.into();
        assert_eq!(Vec2::new(x, y), vector);

        let tmp = Vec2::from([1, 2]);
        assert_eq!(tmp, vector);

        let array = [1, 2, 3, 4, 5, 6];
        let tmp = Vec2::from(&array[0..LENGTH]);
        assert_eq!(tmp, vector);
    }

    #[test]
    fn vector2d_indices_and_iterators() {
        let vector = Vec2::new(1, 2);
        
        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 2);
        
        let mut tmp = vector;
        assert_eq!(*(&mut tmp[0]), 1);
        assert_eq!(*(&mut tmp[1]), 2);

        let mut iter = vector.into_iter();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn vector2d_operations() {
        let vector = Vec2::new(2, 4);

        assert_eq!(vector + 1, Vec2::new(3, 5));
        assert_eq!(1 + vector, Vec2::new(3, 5));
        assert_eq!(vector + Vec2::new(1, 2), Vec2::new(3, 6));

        assert_eq!(vector - 1, Vec2::new(1, 3));
        assert_eq!(vector - Vec2::new(1, 2), Vec2::new(1, 2));

        assert_eq!(vector * 2, Vec2::new(4, 8));
        assert_eq!(2 * vector, Vec2::new(4, 8));
        assert_eq!(vector / 2, Vec2::new(1, 2));
        assert_eq!(vector % 2, Vec2::new(0, 0));

        let mut tmp = vector;
        tmp += 1;
        assert_eq!(tmp, Vec2::new(3, 5));

        let mut tmp = vector;
        tmp += Vec2::new(1, 2);
        assert_eq!(tmp, Vec2::new(3, 6));

        let mut tmp = vector;
        tmp -= 1;
        assert_eq!(tmp, Vec2::new(1, 3));

        let mut tmp = vector;
        tmp -= Vec2::new(1, 2);
        assert_eq!(tmp, Vec2::new(1, 2));

        let mut tmp = vector;
        tmp *= 2;
        assert_eq!(tmp, Vec2::new(4, 8));

        let mut tmp = vector;
        tmp /= 2;
        assert_eq!(tmp, Vec2::new(1, 2));

        let mut tmp = vector;
        tmp %= 2;
        assert_eq!(tmp, Vec2::new(0, 0));

        let tmp = -vector;
        assert_eq!(tmp, Vec2::new(-2, -4));
    }

    #[test]
    fn vector2d_methods() {
        let vector: Vec2<u32> = Vec2::new(2, 2).pow(3);
        assert_eq!(vector, Vec2::new(8, 8));

        let vector = Vec2::new(-1, -1);
        assert_eq!(vector.abs(), -vector);

        let vector = Vec2::new(2.2, 2.8);
        assert!(vector.ceil().is_equal(Vec2::new(3.0, 3.0), EPSILON));

        let vector = Vec2::new(2.2, 2.8);
        assert!(vector.floor().is_equal(Vec2::new(2.0, 2.0), EPSILON));

        let vector = Vec2::from(1.0);
        assert!(vector.mix(Vec2::from(0.0), 0.5).is_equal(Vec2::from(0.5), EPSILON));

        let vector = Vec2::new(2.0, 2.0).pow(-1);
        assert!(vector.is_equal(Vec2::new(0.5, 0.5), EPSILON));

        let vector = Vec2::new(1.3, 2.6).round();
        assert!(vector.is_equal(Vec2::new(1.0, 3.0), EPSILON));

        let vector = Vec2::new(1.0, 2.0);
        assert!(vector.sqr_length().is_equal(5.0, EPSILON));
        assert!(vector.lenght().is_equal(2.23606797749979, EPSILON));
        assert!(vector.normalize().lenght().is_equal(1.0, EPSILON));

        let vector = Vec2::new(5.0, 0.0);
        let vector2 = Vec2::new(0.0, 3.0);
        assert!(vector.normalize().dot(vector2.normalize()).is_equal(0.0, EPSILON));

        let vector = Vec2::new(0, 1);
        let vector: Vec2<f64> = vector.cast();
        assert!(vector.is_equal(Vec2::new(0.0, 1.0), EPSILON));
    }

    #[test]
    fn vector2d_display() {
        let vector = Vec2::new(1, 2);
        assert_eq!(format!("{}", vector), "Vec2(1; 2)");
    }
}