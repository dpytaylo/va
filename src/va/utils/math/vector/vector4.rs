use std::fmt::Display;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

use crate::va::utils::math::is_equal::IsCopyTypeEqual;
use crate::va::utils::cast::Cast;
use crate::va::utils::number::{Number, Float, Integer, Pow};

use super::utils::*;
use super::vector2::Vec2;
use super::vector3::Vec3;

const LENGTH: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vec4<T> 
    where T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_common_vector_methods!(Vec4);

impl<T> Vec4<T>
    where T: Number,
          Vec4<T>: FromIterator<T>,
{
    pub const ZERO: Vec4<T> = Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
    pub const ONE: Vec4<T> = Vec4::new(T::ONE, T::ONE, T::ONE, T::ONE);
    pub const TWO: Vec4<T> = Vec4::new(T::TWO, T::TWO, T::TWO, T::TWO);

    pub const X: Vec4<T> = Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const Y: Vec4<T> = Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    pub const Z: Vec4<T> = Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    pub const W: Vec4<T> = Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);

    pub const MIN: Vec4<T> = Vec4::new(T::MIN, T::MIN, T::MIN, T::MIN);
    pub const MAX: Vec4<T> = Vec4::new(T::MAX, T::MAX, T::MAX, T::MAX);

    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x, y, z, w
        }
    }
}

impl<T> Vec4<T>
    where T: Number,
{
    pub fn xy(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn xz(self) -> Vec2<T> {
        Vec2::new(self.x, self.z)
    }
    
    pub fn xw(self) -> Vec2<T> {
        Vec2::new(self.x, self.w)
    }

    pub fn yz(self) -> Vec2<T> {
        Vec2::new(self.y, self.z)
    }

    pub fn yw(self) -> Vec2<T> {
        Vec2::new(self.y, self.w)
    }

    pub fn zw(self) -> Vec2<T> {
        Vec2::new(self.z, self.w)
    }

    pub fn xyz(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn xyw(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.w)
    }

    pub fn xzw(self) -> Vec3<T> {
        Vec3::new(self.x, self.z, self.w)
    }

    pub fn yzw(self) -> Vec3<T> {
        Vec3::new(self.y, self.z, self.w)
    }
}

impl<T> From<T> for Vec4<T>
    where T: Number,
{
    fn from(value: T) -> Self {
        Self::new(value, value, value, value)
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> 
    where T: Number,
{
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T> From<Vec4<T>> for (T, T, T, T) 
    where T: Number,
{
    fn from(value: Vec4<T>) -> Self {
        (value[0], value[1], value[2], value[3])
    }
}

impl<T> From<Vec4<T>> for [T; 4]
    where T: Number,
{
    fn from(value: Vec4<T>) -> Self {
        [value[0], value[1], value[2], value[3]]
    }
}

impl<T> FromIterator<T> for Vec4<T>
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
            w: iter.next().expect("invalid iterator"),
        } 
    }
}

impl<T> Index<usize> for Vec4<T>
    where T: Number,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T>
    where T: Number,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("invalid index value"),
        }
    }
}

impl<T> Display for Vec4<T>
    where T: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec4({}; {}; {}; {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{cast::Cast, math::vector::vector2::Vec2};

    use super::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn vector4d_comparison() {
        let a = Vec4 {
            x: 20,
            y: 54,
            z: 93,
            w: 24,
        };

        let b = Vec4 {
            x: 20,
            y: 54,
            z: 93,
            w: 24,
        };

        let c = Vec4 {
            x: 334,
            y: 54,
            z: 89,
            w: 31,
        };

        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_ne!(a, c);

        let a = Vec4 {
            x: 20.0,
            y: 54.0,
            z: 93.0,
            w: 24.0,
        };

        let b = Vec4 {
            x: 20.0,
            y: 54.0,
            z: 93.0,
            w: 24.0,
        };

        let c = Vec4 {
            x: 334.0,
            y: 54.0,
            z: 89.0,
            w: 31.0,
        };

        assert!(a.is_equal(b, EPSILON));
        assert!(!a.is_equal(c, EPSILON));

        assert!(a < c);
    }

    #[test]
    fn vector4d_initialization() {
        let vector = Vec4::new(1, 2, 3, 4);
        assert_eq!(vector, Vec4 {x: 1, y: 2, z: 3, w: 4});

        let tmp = Vec4::from(1);
        assert_eq!(tmp, Vec4::new(1, 1, 1, 1));

        let tmp: Vec4<i32> = Vec4::from(Vec4::new(1i8, 2, 3, 4));
        assert_eq!(tmp, vector);

        let tmp: Vec4<i32> = Vec4::try_from(Vec4::new(1i64, 2, 3, 4)).unwrap();
        assert_eq!(tmp, vector);

        let tmp: Result<Vec4<u8>, ()> = Vec4::try_from(Vec4::new(-1i32, 2, 3, 4));
        assert!(tmp.is_err());

        let tmp = Vec4::from((1, 2, 3, 4));
        assert_eq!(tmp, vector);

        let (x, y, z, w) = tmp.into();
        assert_eq!(Vec4::new(x, y, z, w), vector);

        let tmp = Vec4::from([1, 2, 3, 4]);
        assert_eq!(tmp, vector);

        let tmp: [i32; 4] = tmp.into();
        assert_eq!(tmp, [1, 2, 3, 4]);

        let array = [1, 2, 3, 4, 5, 6];
        let tmp = Vec4::from(&array[0..LENGTH]);
        assert_eq!(tmp, vector);

        assert_eq!(Vec4::from(i32::MIN), Vec4::MIN);
        assert_eq!(Vec4::from(i32::MAX), Vec4::MAX);
    }

    #[test]
    fn vector4d_indices_and_iterators() {
        let vector = Vec4::new(1, 2, 3, 4);
        
        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 2);
        assert_eq!(vector[2], 3);
        assert_eq!(vector[3], 4);
        
        let mut tmp = vector;
        assert_eq!(*(&mut tmp[0]), 1);
        assert_eq!(*(&mut tmp[1]), 2);
        assert_eq!(*(&mut tmp[2]), 3);
        assert_eq!(*(&mut tmp[3]), 4);

        let mut iter = vector.into_iter();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 4);
        assert!(iter.next().is_none());
    }

    #[test]
    fn vector4d_operations() {
        let vector = Vec4::new(2, 4, 6, 8);

        assert_eq!(vector + 1, Vec4::new(3, 5, 7, 9));
        assert_eq!(1 + vector, Vec4::new(3, 5, 7, 9));
        assert_eq!(vector + Vec4::new(1, 2, 3, 4), Vec4::new(3, 6, 9, 12));

        assert_eq!(vector - 1, Vec4::new(1, 3, 5, 7));
        assert_eq!(vector - Vec4::new(1, 2, 3, 4), Vec4::new(1, 2, 3, 4));

        assert_eq!(vector * 2, Vec4::new(4, 8, 12, 16));
        assert_eq!(2 * vector, Vec4::new(4, 8, 12, 16));
        assert_eq!(vector / 2, Vec4::new(1, 2, 3, 4));
        assert_eq!(vector % 2, Vec4::new(0, 0, 0, 0));

        let mut tmp = vector;
        tmp += 1;
        assert_eq!(tmp, Vec4::new(3, 5, 7, 9));

        let mut tmp = vector;
        tmp += Vec4::new(1, 2, 3, 4);
        assert_eq!(tmp, Vec4::new(3, 6, 9, 12));

        let mut tmp = vector;
        tmp -= 1;
        assert_eq!(tmp, Vec4::new(1, 3, 5, 7));

        let mut tmp = vector;
        tmp -= Vec4::new(1, 2, 3, 4);
        assert_eq!(tmp, Vec4::new(1, 2, 3, 4));

        let mut tmp = vector;
        tmp *= 2;
        assert_eq!(tmp, Vec4::new(4, 8, 12, 16));

        let mut tmp = vector;
        tmp /= 2;
        assert_eq!(tmp, Vec4::new(1, 2, 3, 4));

        let mut tmp = vector;
        tmp %= 2;
        assert_eq!(tmp, Vec4::new(0, 0, 0, 0));

        let tmp = -vector;
        assert_eq!(tmp, Vec4::new(-2, -4, -6, -8));
    }

    #[test]
    fn vector4d_methods() {
        let vector = Vec4::new(1, 2, 3, 4);
        assert_eq!(vector.xy(), Vec2::new(1, 2));
        assert_eq!(vector.xz(), Vec2::new(1, 3));
        assert_eq!(vector.xw(), Vec2::new(1, 4));
        assert_eq!(vector.yz(), Vec2::new(2, 3));
        assert_eq!(vector.yw(), Vec2::new(2, 4));
        assert_eq!(vector.zw(), Vec2::new(3, 4));

        assert_eq!(vector.xyz(), Vec3::new(1, 2, 3));
        assert_eq!(vector.xyw(), Vec3::new(1, 2, 4));
        assert_eq!(vector.xzw(), Vec3::new(1, 3, 4));
        assert_eq!(vector.yzw(), Vec3::new(2, 3, 4));

        let vector: Vec4<u32> = Vec4::new(2, 2, 2, 2).pow(3);
        assert_eq!(vector, Vec4::new(8, 8, 8, 8));

        let vector = Vec4::new(-1, -1, -1, -1);
        assert_eq!(vector.abs(), -vector);

        let vector = Vec4::new(2.2, 2.8, 2.8, 2.8);
        assert!(vector.ceil().is_equal(Vec4::new(3.0, 3.0, 3.0, 3.0), EPSILON));

        let vector = Vec4::new(2.2, 2.8, 2.8, 2.8);
        assert!(vector.floor().is_equal(Vec4::new(2.0, 2.0, 2.0, 2.0), EPSILON));

        let vector = Vec4::from(1.0);
        assert!(vector.mix(Vec4::from(0.0), 0.5).is_equal(Vec4::from(0.5), EPSILON));

        let vector = Vec4::new(2.0, 2.0, 2.0, 2.0).pow(-1);
        assert!(vector.is_equal(Vec4::new(0.5, 0.5, 0.5, 0.5), EPSILON));

        let vector = Vec4::new(1.3, 2.6, 3.0, 4.9).round();
        assert!(vector.is_equal(Vec4::new(1.0, 3.0, 3.0, 5.0), EPSILON));

        let vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(vector.sqr_length().is_equal(30.0, EPSILON));
        assert!(vector.lenght().is_equal(5.477225575051661, EPSILON));
        assert!(vector.normalize().lenght().is_equal(1.0, EPSILON));

        let vector = Vec4::new(5.0, 0.0, 0.0, 0.0);
        let vector2 = Vec4::new(0.0, 3.0, 0.0, 0.0);
        assert!(vector.normalize().dot(vector2.normalize()).is_equal(0.0, EPSILON));

        let vector = Vec4::new(0, 1, 2, 3);
        let vector: Vec4<f64> = vector.cast();
        assert!(vector.is_equal(Vec4::new(0.0, 1.0, 2.0, 3.0), EPSILON));
    }

    #[test]
    fn vector4d_display() {
        let vector = Vec4::new(1, 2, 3, 4);
        assert_eq!(format!("{}", vector), "Vec4(1; 2; 3; 4)");
    }
}