use std::fmt::Display;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

use crate::va::utils::math::is_equal::IsCopyTypeEqual;
use crate::va::utils::cast::Cast;
use crate::va::utils::number::{Number, Float, Integer, Pow};

use super::utils::*;
use super::vector2::Vec2;

const LENGTH: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vec3<T> 
    where T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_common_vector_methods!(Vec3);

impl<T> Vec3<T>
    where T: Number,
          Vec3<T>: FromIterator<T>,
{
    pub const ZERO: Vec3<T> = Vec3::new(T::ZERO, T::ZERO, T::ZERO);
    pub const ONE: Vec3<T> = Vec3::new(T::ONE, T::ONE, T::ONE);
    pub const TWO: Vec3<T> = Vec3::new(T::TWO, T::TWO, T::TWO);

    pub const X: Vec3<T> = Vec3::new(T::ONE, T::ZERO, T::ZERO);
    pub const Y: Vec3<T> = Vec3::new(T::ZERO, T::ONE, T::ZERO);
    pub const Z: Vec3<T> = Vec3::new(T::ZERO, T::ZERO, T::ONE);

    pub const MIN: Vec3<T> = Vec3::new(T::MIN, T::MIN, T::MIN);
    pub const MAX: Vec3<T> = Vec3::new(T::MAX, T::MAX, T::MAX);

    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            x, y, z
        }
    }
}

impl<T> Vec3<T>
    where T: Number,
{
    pub fn xy(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn xz(self) -> Vec2<T> {
        Vec2::new(self.x, self.z)
    }

    pub fn yz(self) -> Vec2<T> {
        Vec2::new(self.y, self.z)
    }
}

impl<T> Vec3<T>
    where T: Number + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross(self, vector: Self) -> Self {
        Vec3::new(
            self.y * vector.z - self.z * vector.y,
            self.x * vector.z - self.z * vector.x,
            self.x * vector.y - self.y * vector.x,
        )
    }
}

impl<T> From<T> for Vec3<T>
    where T: Number,
{
    fn from(value: T) -> Self {
        Self::new(value, value, value)
    }
}

impl<T> From<(T, T, T)> for Vec3<T> 
    where T: Number,
{
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T> From<Vec3<T>> for (T, T, T) 
    where T: Number,
{
    fn from(value: Vec3<T>) -> Self {
        (value[0], value[1], value[2])
    }
}

impl<T> FromIterator<T> for Vec3<T>
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
            z: iter.next().expect("invalid iterator"),
        } 
    }
}

impl<T> Index<usize> for Vec3<T>
    where T: Number,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T>
    where T: Number,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> Display for Vec3<T>
    where T: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}; {}; {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn vector3d_comparison() {
        let a = Vec3 {
            x: 20,
            y: 54,
            z: 93,
        };

        let b = Vec3 {
            x: 20,
            y: 54,
            z: 93,
        };

        let c = Vec3 {
            x: 334,
            y: 54,
            z: 89,
        };

        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_ne!(a, c);

        let a = Vec3 {
            x: 20.0,
            y: 54.0,
            z: 93.0,
        };

        let b = Vec3 {
            x: 20.0,
            y: 54.0,
            z: 93.0,
        };

        let c = Vec3 {
            x: 334.0,
            y: 54.0,
            z: 89.0,
        };

        assert!(a.is_equal(b, EPSILON));
        assert!(!a.is_equal(c, EPSILON));

        assert!(a < c);
    }

    #[test]
    fn vector3d_initialization() {
        let vector = Vec3::new(1, 2, 3);
        assert_eq!(vector, Vec3 {x: 1, y: 2, z: 3});

        let tmp = Vec3::from(1);
        assert_eq!(tmp, Vec3::new(1, 1, 1));

        let tmp: Vec3<i32> = Vec3::from(Vec3::new(1i8, 2, 3));
        assert_eq!(tmp, vector);

        let tmp: Vec3<i32> = Vec3::try_from(Vec3::new(1i64, 2, 3)).unwrap();
        assert_eq!(tmp, vector);

        let tmp: Result<Vec3<u8>, ()> = Vec3::try_from(Vec3::new(-1i32, 2, 3));
        assert!(tmp.is_err());

        let tmp = Vec3::from((1, 2, 3));
        assert_eq!(tmp, vector);

        let (x, y, z) = tmp.into();
        assert_eq!(Vec3::new(x, y, z), vector);

        let tmp = Vec3::from([1, 2, 3]);
        assert_eq!(tmp, vector);

        let array = [1, 2, 3, 4, 5, 6];
        let tmp = Vec3::from(&array[0..LENGTH]);
        assert_eq!(tmp, vector);

        assert_eq!(Vec3::from(i32::MIN), Vec3::MIN);
        assert_eq!(Vec3::from(i32::MAX), Vec3::MAX);
    }

    #[test]
    fn vector3d_indices_and_iterators() {
        let vector = Vec3::new(1, 2, 3);
        
        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 2);
        assert_eq!(vector[2], 3);
        
        let mut tmp = vector;
        assert_eq!(*(&mut tmp[0]), 1);
        assert_eq!(*(&mut tmp[1]), 2);
        assert_eq!(*(&mut tmp[2]), 3);

        let mut iter = vector.into_iter();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn vector3d_operations() {
        let vector = Vec3::new(2, 4, 6);

        assert_eq!(vector + 1, Vec3::new(3, 5, 7));
        assert_eq!(1 + vector, Vec3::new(3, 5, 7));
        assert_eq!(vector + Vec3::new(1, 2, 3), Vec3::new(3, 6, 9));

        assert_eq!(vector - 1, Vec3::new(1, 3, 5));
        assert_eq!(vector - Vec3::new(1, 2, 3), Vec3::new(1, 2, 3));

        assert_eq!(vector * 2, Vec3::new(4, 8, 12));
        assert_eq!(2 * vector, Vec3::new(4, 8, 12));
        assert_eq!(vector / 2, Vec3::new(1, 2, 3));
        assert_eq!(vector % 2, Vec3::new(0, 0, 0));

        let mut tmp = vector;
        tmp += 1;
        assert_eq!(tmp, Vec3::new(3, 5, 7));

        let mut tmp = vector;
        tmp += Vec3::new(1, 2, 3,);
        assert_eq!(tmp, Vec3::new(3, 6, 9));

        let mut tmp = vector;
        tmp -= 1;
        assert_eq!(tmp, Vec3::new(1, 3, 5));

        let mut tmp = vector;
        tmp -= Vec3::new(1, 2, 3);
        assert_eq!(tmp, Vec3::new(1, 2, 3));

        let mut tmp = vector;
        tmp *= 2;
        assert_eq!(tmp, Vec3::new(4, 8, 12));

        let mut tmp = vector;
        tmp /= 2;
        assert_eq!(tmp, Vec3::new(1, 2, 3));

        let mut tmp = vector;
        tmp %= 2;
        assert_eq!(tmp, Vec3::new(0, 0, 0));

        let tmp = -vector;
        assert_eq!(tmp, Vec3::new(-2, -4, -6));
    }

    #[test]
    fn vector3d_methods() {
        let vector = Vec3::new(1, 2, 3);
        assert_eq!(vector.xy(), Vec2::new(1, 2));
        assert_eq!(vector.xz(), Vec2::new(1, 3));
        assert_eq!(vector.yz(), Vec2::new(2, 3));

        let vector: Vec3<u32> = Vec3::new(2, 2, 2).pow(3);
        assert_eq!(vector, Vec3::new(8, 8, 8));

        let vector = Vec3::new(-1, -1, -1);
        assert_eq!(vector.abs(), -vector);

        let vector = Vec3::new(2.2, 2.8, 2.8);
        assert!(vector.ceil().is_equal(Vec3::new(3.0, 3.0, 3.0), EPSILON));

        let vector = Vec3::new(2.2, 2.8, 2.8);
        assert!(vector.floor().is_equal(Vec3::new(2.0, 2.0, 2.0), EPSILON));

        let vector = Vec3::from(1.0);
        assert!(vector.mix(Vec3::from(0.0), 0.5).is_equal(Vec3::from(0.5), EPSILON));

        let vector = Vec3::new(2.0, 2.0, 2.0).pow(-1);
        assert!(vector.is_equal(Vec3::new(0.5, 0.5, 0.5), EPSILON));

        let vector = Vec3::new(1.3, 2.6, 3.0).round();
        assert!(vector.is_equal(Vec3::new(1.0, 3.0, 3.0), EPSILON));

        let vector = Vec3::new(12, 34, 53).cross(Vec3::new(75, 24, 12));
        assert_eq!(vector, Vec3::new(-864, -3831, -2262));

        let vector = Vec3::new(1.0, 2.0, 3.0);
        assert!(vector.sqr_length().is_equal(14.0, EPSILON));
        assert!(vector.lenght().is_equal(3.7416573867739413, EPSILON));
        assert!(vector.normalize().lenght().is_equal(1.0, EPSILON));

        let vector = Vec3::new(5.0, 0.0, 0.0);
        let vector2 = Vec3::new(0.0, 3.0, 0.0);
        assert!(vector.normalize().dot(vector2.normalize()).is_equal(0.0, EPSILON));

        let vector = Vec3::new(0, 1, 2);
        let vector: Vec3<f64> = vector.cast();
        assert!(vector.is_equal(Vec3::new(0.0, 1.0, 2.0), EPSILON));
    }

    #[test]
    fn vector3d_display() {
        let vector = Vec3::new(1, 2, 3);
        assert_eq!(format!("{}", vector), "Vec3(1; 2; 3)");
    }
}