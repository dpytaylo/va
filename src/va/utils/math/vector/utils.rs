macro_rules! impl_vector_from {
    ($vector:ident, $a:ty, $($n:ty),*) => {
        $(
            impl From<$vector<$n>> for $vector<$a> {
                fn from(value: $vector<$n>) -> Self {
                    value.into_iter().map(|val| val.into()).collect()
                }
            }
        )*
    };
}
pub(crate) use impl_vector_from;

macro_rules! impl_vector_cast {
    ($vector:ident, $a:ty, $($n:ty),*) => {
        $(
            impl crate::utils::cast::Cast<$vector<$n>> for $vector<$a> {
                fn cast(self) -> $vector<$n> {
                    self.into_iter().map(|val| val as $n).collect()
                }
            }
        )*
    };
}
pub(crate) use impl_vector_cast;

macro_rules! impl_vector_casts {
    ($vector:ident, $($n:ty),*) => {
        $(
            super::utils::impl_vector_cast!($vector, $n, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
        )*
    };
}
pub(crate) use impl_vector_casts;

/* From TryFrom source */
// no possible bounds violation
macro_rules! impl_vector_try_from_unbounded {
    ($vector:ident, $source:ty, $($n:ty),*) => {$(
        impl TryFrom<$vector<$source>> for $vector<$n> {
            type Error = ();

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: $vector<$source>) -> Result<Self, Self::Error> {
                Ok(value.cast())
            }
        }
    )*}
}
pub(crate) use impl_vector_try_from_unbounded;

// only negative bounds
macro_rules! impl_vector_try_from_lower_bounded {
    ($vector:ident, $source:ty, $($n:ty),*) => {$(
        impl TryFrom<$vector<$source>> for $vector<$n> {
            type Error = ();

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: $vector<$source>) -> Result<Self, Self::Error> {
                for val in value {
                    if val < 0 {
                        return Err(());
                    }
                }

                Ok(value.cast())
            }
        }
    )*}
}
pub(crate) use impl_vector_try_from_lower_bounded;

// unsigned to signed (only positive bound)
macro_rules! impl_vector_try_from_upper_bounded {
    ($vector:ident, $source:ty, $($n:ty),*) => {$(
        impl TryFrom<$vector<$source>> for $vector<$n> {
            type Error = ();

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: $vector<$source>) -> Result<Self, Self::Error> {
                for val in value {
                    if val > (<$n>::MAX as $source) {
                        return Err(());
                    }
                }

                Ok(value.cast())
            }
        }
    )*}
}
pub(crate) use impl_vector_try_from_upper_bounded;

// all other cases
macro_rules! impl_vector_try_from_both_bounded {
    ($vector:ident, $source:ty, $($n:ty),*) => {$(
        impl TryFrom<$vector<$source>> for $vector<$n> {
            type Error = ();

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: $vector<$source>) -> Result<Self, Self::Error> {
                let min = <$n>::MIN as $source;
                let max = <$n>::MAX as $source;

                for val in value {
                    if val < min || val > max {
                        return Err(());
                    }
                }

                Ok(value.cast())
            }
        }
    )*}
}
pub(crate) use impl_vector_try_from_both_bounded;

macro_rules! vector_rev {
    ($mac:ident, $vector:ident, $source:ty, $($target:ty),*) => {$(
        $mac!($vector, $target, $source);
    )*}
}
pub(crate) use vector_rev;

// macro_rules! impl_vector_try_from {
//     ($vector:ident, $a:ty, $($n:ty),*) => {
//         $(
//             impl TryFrom<$vector<$n>> for $vector<$a> {
//                 type Error = std::num::TryFromIntError;
//                 fn try_from(value: $vector<$n>) -> Result<Self, Self::Error> {
//                     for val in value {
//                         if !(<$a>::MIN <= val && val <= <$a>::MAX) {
//                             return Err(std::num::TryFromIntError);
//                         }
//                     }

//                     Ok(value.into_iter().map(|val| val as $a).collect())
//                 }
//             }
//         )*
//     };
// }
// pub(crate) use impl_vector_try_from;

macro_rules! impl_operators {
    ($vector:ident, $($n:ty),*) => {
        $(
            impl Add<$vector<$n>> for $n {
                type Output = $vector<$n>;

                fn add(self, rhs: $vector<$n>) -> Self::Output {
                    rhs.into_iter().map(|val| self + val).collect()
                }
            }

            impl Mul<$vector<$n>> for $n {
                type Output = $vector<$n>;

                fn mul(self, rhs: $vector<$n>) -> Self::Output {
                    rhs.into_iter().map(|val| self * val).collect()
                }
            }
        )*
    }
}
pub(crate) use impl_operators;

macro_rules! impl_common_vector_methods {
    ($vector:ident) => {
        impl<T> $vector<T>
            where T: Number,
        {
            pub fn dot(self, vector: Self) -> T {
                self.into_iter()
                    .zip(vector.into_iter())
                    .map(|(x, y)| x * y)
                    .sum()
            }

            #[warn(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                LENGTH
            }
        }

        impl<T> $vector<T>
            where T: Number,
        {
            pub fn abs(self) -> Self {
                self.into_iter().map(|val| val.abs()).collect()
            }
        }

        impl<T> $vector<T>
            where T: Number + Float,
        {
            pub fn ceil(self) -> Self {
                self.into_iter().map(|val| val.ceil()).collect()
            }
        
            pub fn floor(self) -> Self {
                self.into_iter().map(|val| val.floor()).collect()
            }

            pub fn round(self) -> Self {
                self.into_iter().map(|val| val.round()).collect()
            }
        }

        impl<T> $vector<T>
            where T: Number + Float + Sub<Output = T> + Mul<Output = T>,
                  $vector<T>: FromIterator<<T as std::ops::Add>::Output>,
        {
            pub fn mix(self, other: Self, k: T) -> Self {
                self * (T::ONE - k) + other * k
            }
        }

        impl<T> $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<T>,
        {
            pub fn sqr_length(&self) -> T {
                self.into_iter().map(|val| val.sqr()).sum::<T>()
            }
        }

        impl<T> From<[T; LENGTH]> for $vector<T>
            where T: Number,
        {
            fn from(value: [T; LENGTH]) -> Self {
                value.into_iter().collect()
            }
        }

        impl<T> From<&[T]> for $vector<T>
            where T: Number,
        {
            /// # Panic:
            ///
            /// If value doesn't bound for vector size
            fn from(value: &[T]) -> Self {
                value.iter().copied().collect()
            }
        }

        impl_vector_from!($vector, u16, u8);
        impl_vector_from!($vector, u32, u8, u16);
        impl_vector_from!($vector, u64, u8, u16, u32);
        impl_vector_from!($vector, u128, u8, u16, u32, u64);

        impl_vector_from!($vector, i16, u8, i8);
        impl_vector_from!($vector, i32, u8, u16, i8, i16);
        impl_vector_from!($vector, i64, u8, u16, u32, i8, i16, i32);
        impl_vector_from!($vector, i128, u8, u16, u32, u64, i8, i16, i32, i64);

        impl_vector_from!($vector, f32, u8, u16, i8, i16);
        impl_vector_from!($vector, f64, u8, u16, u32, i8, i16, i32, f32);

        impl_vector_from!($vector, usize, u8, u16);
        impl_vector_from!($vector, isize, u8, i8, i16);

        impl_vector_casts!($vector, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        // intra-sign conversions
        impl_vector_try_from_upper_bounded!($vector, u16, u8);
        impl_vector_try_from_upper_bounded!($vector, u32, u16, u8);
        impl_vector_try_from_upper_bounded!($vector, u64, u32, u16, u8);
        impl_vector_try_from_upper_bounded!($vector, u128, u64, u32, u16, u8);
        
        impl_vector_try_from_both_bounded!($vector, i16, i8);
        impl_vector_try_from_both_bounded!($vector, i32, i16, i8);
        impl_vector_try_from_both_bounded!($vector, i64, i32, i16, i8);
        impl_vector_try_from_both_bounded!($vector, i128, i64, i32, i16, i8);

        // unsigned-to-signed
        impl_vector_try_from_upper_bounded!($vector, u8, i8);
        impl_vector_try_from_upper_bounded!($vector, u16, i8, i16);
        impl_vector_try_from_upper_bounded!($vector, u32, i8, i16, i32);
        impl_vector_try_from_upper_bounded!($vector, u64, i8, i16, i32, i64);
        impl_vector_try_from_upper_bounded!($vector, u128, i8, i16, i32, i64, i128);

        // signed-to-unsigned
        impl_vector_try_from_lower_bounded!($vector, i8, u8, u16, u32, u64, u128);
        impl_vector_try_from_lower_bounded!($vector, i16, u16, u32, u64, u128);
        impl_vector_try_from_lower_bounded!($vector, i32, u32, u64, u128);
        impl_vector_try_from_lower_bounded!($vector, i64, u64, u128);
        impl_vector_try_from_lower_bounded!($vector, i128, u128);
        impl_vector_try_from_both_bounded!($vector, i16, u8);
        impl_vector_try_from_both_bounded!($vector, i32, u16, u8);
        impl_vector_try_from_both_bounded!($vector, i64, u32, u16, u8);
        impl_vector_try_from_both_bounded!($vector, i128, u64, u32, u16, u8);

        // usize/isize
        impl_vector_try_from_upper_bounded!($vector, usize, isize);
        impl_vector_try_from_lower_bounded!($vector, isize, usize);

        #[cfg(target_pointer_width = "16")]
        mod ptr_try_from_impls {
            use super::*;

            impl_vector_try_from_upper_bounded!($vector, usize, u8);
            impl_vector_try_from_unbounded!($vector, usize, u16, u32, u64, u128);
            impl_vector_try_from_upper_bounded!($vector, usize, i8, i16);
            impl_vector_try_from_unbounded!($vector, usize, i32, i64, i128);

            impl_vector_try_from_both_bounded!($vector, isize, u8);
            impl_vector_try_from_lower_bounded!($vector, isize, u16, u32, u64, u128);
            impl_vector_try_from_both_bounded!($vector, isize, i8);
            impl_vector_try_from_unbounded!($vector, isize, i16, i32, i64, i128);

            vector_rev!(impl_vector_try_from_upper_bounded, $vector, usize, u32, u64, u128);
            vector_rev!(impl_vector_try_from_lower_bounded, $vector, usize, i8, i16);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, usize, i32, i64, i128);

            vector_rev!(impl_vector_try_from_upper_bounded, $vector, isize, u16, u32, u64, u128);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, isize, i32, i64, i128);
        }

        #[cfg(target_pointer_width = "32")]
        mod ptr_try_from_impls {
            use super::*;

            impl_vector_try_from_upper_bounded!($vector, usize, u8, u16);
            impl_vector_try_from_unbounded!($vector, usize, u32, u64, u128);
            impl_vector_try_from_upper_bounded!($vector, usize, i8, i16, i32);
            impl_vector_try_from_unbounded!($vector, usize, i64, i128);

            impl_vector_try_from_both_bounded!($vector, isize, u8, u16);
            impl_vector_try_from_lower_bounded!($vector, isize, u32, u64, u128);
            impl_vector_try_from_both_bounded!($vector, isize, i8, i16);
            impl_vector_try_from_unbounded!($vector, isize, i32, i64, i128);

            vector_rev!(impl_vector_try_from_unbounded, $vector, usize, u32);
            vector_rev!(impl_vector_try_from_upper_bounded, $vector, usize, u64, u128);
            vector_rev!(impl_vector_try_from_lower_bounded, $vector, usize, i8, i16, i32);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, usize, i64, i128);

            vector_rev!(impl_vector_try_from_unbounded, $vector, isize, u16);
            vector_rev!(impl_vector_try_from_upper_bounded, $vector, isize, u32, u64, u128);
            vector_rev!(impl_vector_try_from_unbounded, $vector, isize, i32);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, isize, i64, i128);
        }

        #[cfg(target_pointer_width = "64")]
        mod ptr_try_from_impls {
            use super::*;

            impl_vector_try_from_upper_bounded!($vector, usize, u8, u16, u32);
            impl_vector_try_from_unbounded!($vector, usize, u64, u128);
            impl_vector_try_from_upper_bounded!($vector, usize, i8, i16, i32, i64);
            impl_vector_try_from_unbounded!($vector, usize, i128);

            impl_vector_try_from_both_bounded!($vector, isize, u8, u16, u32);
            impl_vector_try_from_lower_bounded!($vector, isize, u64, u128);
            impl_vector_try_from_both_bounded!($vector, isize, i8, i16, i32);
            impl_vector_try_from_unbounded!($vector, isize, i64, i128);

            vector_rev!(impl_vector_try_from_unbounded, $vector, usize, u32, u64);
            vector_rev!(impl_vector_try_from_upper_bounded, $vector, usize, u128);
            vector_rev!(impl_vector_try_from_lower_bounded, $vector, usize, i8, i16, i32, i64);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, usize, i128);

            vector_rev!(impl_vector_try_from_unbounded, $vector, isize, u16, u32);
            vector_rev!(impl_vector_try_from_upper_bounded, $vector, isize, u64, u128);
            vector_rev!(impl_vector_try_from_unbounded, $vector, isize, i32, i64);
            vector_rev!(impl_vector_try_from_both_bounded, $vector, isize, i128);
        }

        pub struct IntoIter<T>
            where T: Number,
        {
            vector: $vector<T>,
            index: usize,
        }

        impl<T> Iterator for IntoIter<T>
            where T: Number,
        {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= LENGTH {
                    return None;
                }

                self.index += 1;
                Some(self.vector[self.index - 1])
            }
        }

        impl<T> IntoIterator for $vector<T>
            where T: Number,
        {
            type Item = T;
            type IntoIter = IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter {
                    vector: self,
                    index: 0,
                }
            }
        }

        impl<T> Add<T> for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Add>::Output>,
        {
            type Output = Self;

            fn add(self, rhs: T) -> Self::Output {
                self.into_iter().map(|val| val + rhs).collect()
            }
        }

        impl<T> Add for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Add>::Output>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                self.into_iter()
                    .zip(rhs)
                    .map(|(left, right)| left + right)
                    .collect()
            }
        }

        impl<T> AddAssign<T> for $vector<T>
            where T: Number + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: T) {
                *self = *self + rhs;
            }
        }

        impl<T> AddAssign for $vector<T>
            where T: Number + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<T> Sub<T> for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Sub>::Output>,
        {
            type Output = Self;

            fn sub(self, rhs: T) -> Self::Output {
                self.into_iter().map(|val| val - rhs).collect()
            }
        }

        impl<T> Sub for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Sub>::Output>,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self.into_iter()
                    .zip(rhs)
                    .map(|(left, right)| left - right)
                    .collect()
            }
        }

        impl<T> SubAssign<T> for $vector<T>
            where T: Number + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: T) {
                *self = *self - rhs;
            }
        }

        impl<T> SubAssign for $vector<T>
            where T: Number + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<T> Neg for $vector<T>
            where T: Number + Neg,
                  $vector<T>: FromIterator<<T as Neg>::Output>,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self.into_iter().map(|val| -val).collect()
            }
        }

        impl<T> Mul<T> for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Mul>::Output>,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                self.into_iter().map(|val| val * rhs).collect()
            }
        }

        impl<T> MulAssign<T> for $vector<T>
            where T: Number + Mul<Output = T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl<T> Div<T> for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Div>::Output>,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                self.into_iter().map(|val| val / rhs).collect()
            }
        }

        impl<T> DivAssign<T> for $vector<T>
            where T: Number + Div<Output = T>,
        {
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl<T> Rem<T> for $vector<T>
            where T: Number,
                  $vector<T>: FromIterator<<T as Rem>::Output>,
        {
            type Output = Self;

            fn rem(self, rhs: T) -> Self::Output {
                self.into_iter().map(|val| val % rhs).collect()
            }
        }

        impl<T> RemAssign<T> for $vector<T>
            where T: Number + Rem<Output = T>,
        {
            fn rem_assign(&mut self, rhs: T) {
                *self = *self % rhs;
            }
        }

        super::utils::impl_operators!(
            $vector, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
        );

        impl<T> IsCopyTypeEqual for $vector<T>
            where T: Number + Float + Into<f64>,
        {
            fn is_equal(self, other: Self, epsilon: f64) -> bool {
                for i in 0..LENGTH {
                    if !(self[i].into()).is_equal(other[i].into(), epsilon) {
                        return false;
                    }
                }

                true
            }
        }

        impl<T> Pow<u32> for $vector<T>
            where T: Number + Integer,
        {
            fn pow(self, n: u32) -> Self {
                self.into_iter().map(|val| val.powi(n)).collect()
            }
        }

        impl<T> Pow<i32> for $vector<T>
            where T: Number + Float,
        {
            fn pow(self, n: i32) -> Self {
                self.into_iter().map(|val| val.powi(n)).collect()
            }
        }

        impl<T> $vector<T>
            where T: Number + Float,
                  $vector<T>: FromIterator<<T as Div>::Output>,
        {
            pub fn lenght(&self) -> T {
                self.sqr_length().sqrt()
            }

            pub fn normalize(self) -> Self {
                let length = self.lenght();
                self.into_iter().map(|val| val / length).collect()
            }
        }
    };
}
pub(crate) use impl_common_vector_methods;