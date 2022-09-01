// pub trait PrimitiveNumber<T = Self> {
//     fn zero() -> T;
//     fn one() -> T;
//     fn two() -> T;
// }

// macro_rules! impl_primivite_number_methods {
//     ($($n:ty),*) => {
//         $(
//             impl PrimitiveNumber<$n> for $n {
//                 fn zero() -> $n {
//                     0 as $n
//                 }

//                 fn one() -> $n {
//                     1 as $n
//                 }

//                 fn two() -> $n {
//                     2 as $n
//                 }
//             }
//         )*
//     };
// }

// impl_primivite_number_methods!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn function<T>(value: T) -> bool 
//         where T: PrimitiveNumber + std::cmp::PartialEq,
//     {
//         value == T::zero()
//     }

//     #[test]
//     fn primitive_number_test() {
//         assert!(function(0));
//     }
// }