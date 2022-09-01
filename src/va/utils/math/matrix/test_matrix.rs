// use std::mem::MaybeUninit;
// use std::ops::{Add, Index, IndexMut, Mul, Sub};

// use crate::utils::number::{Number, Float};
// use crate::utils::primitive_number::PrimitiveNumber;
// use crate::utils::math::vector::vector3d::Vec3d;
// use crate::utils::math::vector::vector4d::Vec4d;
// use crate::utils::math::is_equal::IsCopyTypeEqual;

// #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
// pub struct Mat<T, const COLUMN_COUNT: usize, const ROW_COUNT: usize, const LENGTH: usize>
//     where T: Number,
// {
//     data: [T; LENGTH],
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize
// > Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH>
//     where T: Number,
//           //Mat<T, COLUMN, ROW, LENGTH>: Mul<Output = Mat<T, COLUMN, ROW, LENGTH>>,
// {
//     const COLUMN_COUNT: usize = COLUMN_COUNT;
//     const ROW_COUNT: usize = ROW_COUNT;
//     const LENGTH: usize = LENGTH;

//     /// Returns identity matrix
//     pub fn new(data: [T; LENGTH]) -> Self {
//         Self {
//             data
//         }
//     }

//     pub fn get(&self, row: usize, column: usize) -> T {
//         self.data[COLUMN_COUNT * row + column]
//     }

//     pub fn get_mut(&mut self, row: usize, column: usize) -> &mut T {
//         &mut self.data[COLUMN_COUNT * row + column]
//     }

//     pub fn view() -> Self {
//         todo!(); // TODO
//     }
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize
// >  FromIterator<T> for Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH> 
//     where T: Number,
// {
//     fn from_iter<I>(iter: I) -> Self
//         where I: IntoIterator<Item = T>,
//     {
//         let iter = iter.into_iter();
//         let mut data: [MaybeUninit<T>; LENGTH] = MaybeUninit::uninit_array();

//         let mut i = 0;
//         for item in iter {
//             if i >= Self::LENGTH {
//                 panic!("iterator is bigger than matrix length");
//             }

//             data[i].write(item);
//             i += 1;
//         }

//         if i >= Self::LENGTH {
//             panic!("iterator is less than matrix length");
//         }

//         let data = unsafe {
//             MaybeUninit::array_assume_init(data)
//         };

//         Self {
//             data
//         }
//     }
// }

// impl<T> Mat<T, 4, 4, 16>
//     where T: Number + PrimitiveNumber,
//           Mat<T, 4, 4, 16>: Mul<Output = Mat<T, 4, 4, 16>>,
// {
//     pub fn translate(self, vector: Vec3d<T>) -> Self {
//         let translate = Self::new([
//             T::one(), T::zero(), T::zero(), vector.x,
//             T::zero(), T::one(), T::zero(), vector.y,
//             T::zero(), T::zero(), T::one(), vector.z,
//             T::zero(), T::zero(), T::zero(), T::one(),
//         ]);

//         self * translate
//     }
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize
// > Index<usize> for Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH>
//     where T: Number,
// {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize,
// > IndexMut<usize> for Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH>
//     where T: Number,
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }

// pub struct IntoIter<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize,
// >
//     where T: Number,
// {
//     matrix: Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH>,
//     index: usize,
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize,
// > Iterator for IntoIter<T, COLUMN_COUNT, ROW_COUNT, LENGTH>
//     where T: Number,
// {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= LENGTH {
//             return None;
//         }

//         self.index += 1;
//         Some(self.matrix[self.index - 1])
//     }
// }

// impl<
//     T, 
//     const COLUMN_COUNT: usize, 
//     const ROW_COUNT: usize, 
//     const LENGTH: usize,
// > IntoIterator for Mat<T, COLUMN_COUNT, ROW_COUNT, LENGTH>
//     where T: Number,
// {
//     type Item = T;
//     type IntoIter = IntoIter<T, COLUMN_COUNT, ROW_COUNT, LENGTH>;

//     fn into_iter(self) -> Self::IntoIter {
//         Self::IntoIter {
//             matrix: self,
//             index: 0,
//         }
//     }
// }

// // impl<T> Default for Mat4x4<T>
// //     where T: Number + PrimitiveNumber,
// // {
// //     /// Returns identity matrix
// //     fn default() -> Self {
// //         Self {
// //             data: [
// //                 T::one(), T::zero(), T::zero(), T::zero(),
// //                 T::zero(), T::one(), T::zero(), T::zero(),
// //                 T::zero(), T::zero(), T::one(), T::zero(),
// //                 T::zero(), T::zero(), T::zero(), T::one(),
// //             ],
// //         }
// //     }
// // }

// // impl<T> From<T> for Mat4x4<T>
// //     where T: Number,
// // {
// //     fn from(value: T) -> Self {
// //         Self {
// //             data: [
// //                 value, value, value, value,
// //                 value, value, value, value,
// //                 value, value, value, value,
// //                 value, value, value, value,
// //             ],
// //         }
// //     }
// // }

// // impl<T> Index<usize> for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = T;

// //     fn index(&self, index: usize) -> &Self::Output {
// //         &self.data[index]
// //     }
// // }

// // impl<T> IndexMut<usize> for Mat4x4<T>
// //     where T: Number,
// // {
// //     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
// //         &mut self.data[index]
// //     }
// // }

// // impl<T> Add<T> for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = Self;

// //     fn add(mut self, rhs: T) -> Self::Output {
// //         self.data.iter_mut().for_each(|val| *val += rhs);
// //         self
// //     }
// // }

// // impl<T> Add for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = Self;

// //     fn add(mut self, rhs: Self) -> Self::Output {
// //         for i in 0..LENGTH {
// //             self.data[i] += rhs[i];
// //         }
// //         self
// //     }
// // }

// // impl<T> Sub<T> for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = Self;

// //     fn sub(mut self, rhs: T) -> Self::Output {
// //         self.data.iter_mut().for_each(|val| *val -= rhs);
// //         self
// //     }
// // }

// // impl<T> Sub for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = Self;

// //     fn sub(mut self, rhs: Self) -> Self::Output {
// //         for i in 0..LENGTH {
// //             self.data[i] -= rhs[i];
// //         }
// //         self
// //     }
// // }

// // impl<T> Mul<T> for Mat4x4<T>
// //     where T: Number,
// // {
// //     type Output = Self;

// //     fn mul(mut self, rhs: T) -> Self::Output {
// //         self.data.iter_mut().for_each(|val| *val *= rhs);
// //         self
// //     }
// // }

// // impl<T> Mul<Vec4d<T>> for Mat4x4<T>
// //     where T: Number + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
// // {
// //     type Output = Vec4d<T>;

// //     fn mul(self, rhs: Vec4d<T>) -> Self::Output {
// //         let mut vector = Vec4d::default();

// //         for i in 0..DIMENSION {
// //             for j in 0..DIMENSION {
// //                 vector[i] += rhs[j] * self[DIMENSION * i + j];
// //             }
// //         }

// //         vector
// //     }
// // }

// // impl<T> Mul for Mat4x4<T>
// //     where T: Number + PrimitiveNumber + Mul<Output = T>,
// // {
// //     type Output = Self;

// //     fn mul(self, rhs: Self) -> Self::Output {
// //         let mut matrix = Self::from(T::zero());

// //         for y in 0..DIMENSION {
// //             for x in 0..DIMENSION {
// //                 for i in 0..DIMENSION {
// //                     *matrix.get_mut(x, y) += self.get(i, y) * rhs.get(x, i);
// //                 }
// //             }
// //         }

// //         matrix
// //     }
// // }

// // impl<T> IsCopyTypeEqual for Mat4x4<T>
// //     where T: Number + Float + Into<f64>, 
// // {
// //     fn is_equal(self, other: Self, epsilon: f64) -> bool {
// //         for i in 0..LENGTH {
// //             if !(self.data[i].into()).is_equal(other.data[i].into(), epsilon) {
// //                 return false;
// //             }
// //         }

// //         return true;
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EPSILON: f64 = 0.001;

//     #[test]
//     fn matrix_initialization() {
//         let matrix = Mat::<_, 2, 2, 4>::new([1, 2, 3, 4]);
//         assert_eq!(matrix.get(0, 2), 1);
//     }

//     // #[test]
//     // fn matrix4x4_comparison() {
//     //     let matrix = Mat4x4 {
//     //         data: [
//     //             1, 1, 1, 1,
//     //             1, 1, 1, 1,
//     //             1, 1, 1, 1,
//     //             1, 1, 1, 1
//     //         ],
//     //     };
//     //     assert_eq!(matrix, matrix);

//     //     let matrix = Mat4x4 {
//     //         data: [
//     //             1.0, 1.0, 1.0, 1.0,
//     //             1.0, 1.0, 1.0, 1.0,
//     //             1.0, 1.0, 1.0, 1.0,
//     //             1.0, 1.0, 1.0, 1.0
//     //         ],
//     //     };
//     //     assert!(matrix.is_equal(matrix, EPSILON));
//     // }

//     // #[test]
//     // fn matrix4x4_initialization() {
//     //     let matrix = Mat4x4::from(1);
//     //     assert_eq!(matrix, Mat4x4::new([
//     //         1, 1, 1, 1,
//     //         1, 1, 1, 1,
//     //         1, 1, 1, 1,
//     //         1, 1, 1, 1,
//     //     ]));
//     // }

//     // #[test]
//     // fn matrix4x4_indices_and_iterators() {
//     //     let matrix: Mat4x4<i32> = Mat4x4::default();
//     //     assert_eq!(matrix[0], matrix.get(0, 0));
//     //     assert_eq!(matrix[10], matrix.get(2, 2));

//     //     // TODO
//     // }

//     // #[test]
//     // fn matrix4x4_methods() {
//     //     let matrix: Mat4x4<i32> = Mat4x4::default();
//     //     assert_eq!(matrix + 1, Mat4x4::new([
//     //         2, 1, 1, 1,
//     //         1, 2, 1, 1,
//     //         1, 1, 2, 1,
//     //         1, 1, 1, 2,
//     //     ]));

//     //     let tmp = Mat4x4::from(1);
//     //     assert_eq!((matrix + tmp), Mat4x4::new([
//     //         2, 1, 1, 1,
//     //         1, 2, 1, 1,
//     //         1, 1, 2, 1,
//     //         1, 1, 1, 2,
//     //     ]));

//     //     assert_eq!((matrix - 1), Mat4x4::new([
//     //         0, -1, -1, -1,
//     //         -1, 0, -1, -1,
//     //         -1, -1, 0, -1,
//     //         -1, -1, -1, 0,
//     //     ]));

//     //     let tmp = Mat4x4::from(1);
//     //     assert_eq!((matrix - tmp), Mat4x4::new([
//     //         0, -1, -1, -1,
//     //         -1, 0, -1, -1,
//     //         -1, -1, 0, -1,
//     //         -1, -1, -1, 0,
//     //     ]));

//     //     // assert_eq!((matrix * 2), Mat4x4::new([
//     //     //     2, 0, 0, 0,
//     //     //     0, 2, 0, 0,
//     //     //     0, 0, 2, 0,
//     //     //     0, 0, 0, 2,
//     //     // ]));

//     //     let test_mul_matrix = Mat4x4::new([
//     //         3, 4, 1, 4,
//     //         2, 4, 3, 1,
//     //         2, 1, 7, 7,
//     //         9, 6, 4, 5,
//     //     ]);
//     //     let test_mul_matrix_2 = Mat4x4::new([
//     //         5, 4, 3, 4,
//     //         4, 5, 2, 3,
//     //         2, 3, 1, 5,
//     //         7, 6, 1, 2,
//     //     ]);
//     //     assert_eq!(test_mul_matrix * test_mul_matrix_2, Mat4x4::new([
//     //         61, 59, 22, 37,
//     //         39, 43, 18, 37,
//     //         77, 76, 22, 60,
//     //         112, 108, 48, 84,
//     //     ]));
//     // }
// }


// pub trait Matrix: Index<usize> + IndexMut<usize> + From<T> {
//     const LENGTH: usize;


// }