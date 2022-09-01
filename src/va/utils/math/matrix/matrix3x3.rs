use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::utils::math::vector::vector3::Vec3;
use crate::utils::number::{Number, Float};
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector4::Vec4;
use crate::utils::math::is_equal::IsCopyTypeEqual;

const COLUMN_COUNT: usize = 3;
const ROW_COUNT: usize = 3;
const LENGTH: usize = 9;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Mat3x3<T>
    where T: Number,
{
    data: [T; LENGTH]
}

impl<T> Mat3x3<T>
    where T: Number,
          Mat3x3<T>: Mul<Output = Mat3x3<T>>,
{
    /// Returns identity matrix
    pub fn new(data: [T; LENGTH]) -> Self {
        Self {
            data
        }
    }

    pub fn with_scale(vector: Vec2<T>) -> Self {
        Self {
            data: [
                vector.x, T::ZERO, T::ZERO,
                T::ZERO, vector.y, T::ZERO,
                T::ZERO, T::ZERO, T::ONE,
            ],
        }
    }

    pub fn with_translate(vector: Vec2<T>) -> Self {
        Self { 
            data: [
                T::ONE, T::ZERO, vector.x,
                T::ZERO, T::ONE, vector.y,
                T::ZERO, T::ZERO, T::ONE,
            ]
        }
    }

    pub fn get(&self, column: usize, row: usize) -> T {
        self.data[COLUMN_COUNT * row + column]
    }

    pub fn get_mut(&mut self, column: usize, row: usize) -> &mut T {
        &mut self.data[COLUMN_COUNT * row + column]
    }

    // https://github.com/g-truc/glm/blob/master/glm/ext/matrix_transform.inl
    pub fn scale(&mut self, vector: Vec2<T>) {
        self[0] *= vector.x;
        self[4] *= vector.y;
    }

    pub fn translate(&mut self, vector: Vec2<T>) {
        self[2] = vector.x;
        self[5] = vector.y;
    }

    pub fn view() -> Self {
        todo!(); // TODO
    }
}

impl<T> Default for Mat3x3<T>
    where T: Number,
{
    /// Returns identity matrix
    fn default() -> Self {
        Self {
            data: [
                T::ONE, T::ZERO, T::ZERO,
                T::ZERO, T::ONE, T::ZERO,
                T::ZERO, T::ZERO, T::ONE,
            ],
        }
    }
}

impl<T> From<T> for Mat3x3<T>
    where T: Number,
{
    fn from(value: T) -> Self {
        Self {
            data: [
                value, value, value,
                value, value, value,
                value, value, value,
            ],
        }
    }
}

impl<T> Index<usize> for Mat3x3<T>
    where T: Number,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Mat3x3<T>
    where T: Number,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}


impl<T> Add<T> for Mat3x3<T>
    where T: Number,
{
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self.data.iter_mut().for_each(|val| *val += rhs);
        self
    }
}

impl<T> Add for Mat3x3<T>
    where T: Number,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..LENGTH {
            self.data[i] += rhs[i];
        }
        self
    }
}

impl<T> Sub<T> for Mat3x3<T>
    where T: Number,
{
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        self.data.iter_mut().for_each(|val| *val -= rhs);
        self
    }
}

impl<T> Sub for Mat3x3<T>
    where T: Number,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..LENGTH {
            self.data[i] -= rhs[i];
        }
        self
    }
}

impl<T> Mul<T> for Mat3x3<T>
    where T: Number,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.data.iter_mut().for_each(|val| *val *= rhs);
        self
    }
}

impl<T> Mul<Vec3<T>> for Mat3x3<T>
    where T: Number + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let mut vector = Vec3::default();

        for i in 0..ROW_COUNT {
            for j in 0..COLUMN_COUNT {
                vector[i] += rhs[j] * self[COLUMN_COUNT * i + j];
            }
        }

        vector
    }
}

impl<T> Mul for Mat3x3<T>
    where T: Number + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Self::from(T::ZERO);

        for y in 0..COLUMN_COUNT {
            for x in 0..ROW_COUNT {
                for i in 0..COLUMN_COUNT {
                    *matrix.get_mut(x, y) += self.get(i, y) * rhs.get(x, i);
                }
            }
        }

        matrix
    }
}

impl<T> IsCopyTypeEqual for Mat3x3<T>
    where T: Number + Float + Into<f64>, 
{
    fn is_equal(self, other: Self, epsilon: f64) -> bool {
        for i in 0..LENGTH {
            if !(self.data[i].into()).is_equal(other.data[i].into(), epsilon) {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn matrix4x4_comparison() {
        let matrix = Mat3x3 {
            data: [
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
            ],
        };
        assert_eq!(matrix, matrix);

        let matrix = Mat3x3 {
            data: [
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
            ],
        };
        assert!(matrix.is_equal(matrix, EPSILON));
    }

    #[test]
    fn matrix4x4_initialization() {
        let matrix = Mat3x3::from(1);
        assert_eq!(matrix, Mat3x3::new([
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]));
    }

    #[test]
    fn matrix4x4_indices_and_iterators() {
        let matrix: Mat3x3<i32> = Mat3x3::default();
        assert_eq!(matrix[0], matrix.get(0, 0));
        assert_eq!(matrix[4], matrix.get(1, 1));

        // TODO
    }

    #[test]
    fn matrix4x4_methods() {
        let matrix: Mat3x3<i32> = Mat3x3::default();
        assert_eq!(matrix + 1, Mat3x3::new([
            2, 1, 1,
            1, 2, 1,
            1, 1, 2,
        ]));

        let tmp = Mat3x3::from(1);
        assert_eq!((matrix + tmp), Mat3x3::new([
            2, 1, 1,
            1, 2, 1,
            1, 1, 2,
        ]));

        assert_eq!((matrix - 1), Mat3x3::new([
            0, -1, -1,
            -1, 0, -1,
            -1, -1, 0,
        ]));

        let tmp = Mat3x3::from(1);
        assert_eq!((matrix - tmp), Mat3x3::new([
            0, -1, -1,
            -1, 0, -1,
            -1, -1, 0,
        ]));

        // assert_eq!((matrix * 2), Mat3x3::new([
        //     2, 0, 0, 0,
        //     0, 2, 0, 0,
        //     0, 0, 2, 0,
        //     0, 0, 0, 2,
        // ]));

        let test_mul_matrix = Mat3x3::new([
            3, 4, 1,
            2, 4, 3,
            2, 1, 7,
        ]);
        let test_mul_matrix_2 = Mat3x3::new([
            5, 4, 3,
            4, 5, 2,
            2, 3, 1,
        ]);
        assert_eq!(test_mul_matrix * test_mul_matrix_2, Mat3x3::new([
            33, 35, 18,
            32, 37, 17,
            28, 34, 15,
        ]));
    }
}
