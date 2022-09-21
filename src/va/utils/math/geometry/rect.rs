use std::ops::{Add, Sub};

use crate::utils::{math::vector::vector2::Vec2, number::Float};
use crate::utils::number::Number;

use super::{line::{CrossLineResult, self}, point::PointGeometry};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rect<T> 
    where T: Number,
{
    pub p1: Vec2<T>,
    pub p2: Vec2<T>,
}

impl<T> Rect<T>
    where T: Number,
{
    pub fn new(p1: Vec2<T>, p2: Vec2<T>) -> Self {
        Self {
            p1, 
            p2,
        }
    }
}

impl<T> FromIterator<Vec2<T>> for Rect<T>
    where T: Number,
{
    fn from_iter<Iter>(iter: Iter) -> Self
        where Iter: IntoIterator<Item = Vec2<T>>,
    {
        let mut iter = iter.into_iter();
        Self::new(
            iter.next().expect("invalid iterator"), 
            iter.next().expect("invalid iterator"),
        )
    }
}

pub struct IntoIter<T>
    where T: Number,
{
    rect: Rect<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T>
    where T: Number,
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Some(match self.index {
            0 => self.rect.p1,
            1 => self.rect.p2,
            _ => return None,
        });

        self.index += 1;
        point
    }
}

impl<T> IntoIterator for Rect<T>
    where T: Number,
{
    type Item = Vec2<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            rect: self,
            index: 0,
        }
    }
}

macro_rules! impl_rect_from {
    ($a:ty, $($n:ty),*) => {
        $(
            impl From<Rect<$n>> for Rect<$a> {
                fn from(value: Rect<$n>) -> Self {
                    value.into_iter().map(|val| val.into()).collect()
                }
            }
        )*
    };
}

macro_rules! impl_rect_cast {
    ($a:ty, $($n:ty),*) => {
        $(
            impl crate::utils::cast::Cast<Rect<$n>> for Rect<$a> {
                fn cast(self) -> Rect<$n> {
                    self.into_iter().map(|val| val.cast()).collect()
                }
            }
        )*
    };
}

macro_rules! impl_rect_casts {
    ($($n:ty),*) => {
        $(
            impl_rect_cast!($n, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
        )*
    };
}

macro_rules! impl_rect_try_from {
    ($source:ty, $($n:ty),*) => {$(
        impl TryFrom<Rect<$source>> for Rect<$n> {
            type Error = ();

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: Rect<$source>) -> Result<Self, Self::Error> {
                Ok(Rect::new(value.p1.try_into()?, value.p2.try_into()?))
            }
        }
    )*}
}

macro_rules! rect_rev {
    ($mac:ident, $source:ty, $($target:ty),*) => {$(
        $mac!($target, $source);
    )*}
}

impl_rect_from!(u16, u8);
impl_rect_from!(u32, u8, u16);
impl_rect_from!(u64, u8, u16, u32);
impl_rect_from!(u128, u8, u16, u32, u64);

impl_rect_from!(i16, u8, i8);
impl_rect_from!(i32, u8, u16, i8, i16);
impl_rect_from!(i64, u8, u16, u32, i8, i16, i32);
impl_rect_from!(i128, u8, u16, u32, u64, i8, i16, i32, i64);

impl_rect_from!(f32, u8, u16, i8, i16);
impl_rect_from!(f64, u8, u16, u32, i8, i16, i32, f32);

impl_rect_from!(usize, u8, u16);
impl_rect_from!(isize, u8, i8, i16);

impl_rect_casts!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

// intra-sign conversions
impl_rect_try_from!(u16, u8);
impl_rect_try_from!(u32, u16, u8);
impl_rect_try_from!(u64, u32, u16, u8);
impl_rect_try_from!(u128, u64, u32, u16, u8);

impl_rect_try_from!(i16, i8);
impl_rect_try_from!(i32, i16, i8);
impl_rect_try_from!(i64, i32, i16, i8);
impl_rect_try_from!(i128, i64, i32, i16, i8);

// unsigned-to-signed
impl_rect_try_from!(u8, i8);
impl_rect_try_from!(u16, i8, i16);
impl_rect_try_from!(u32, i8, i16, i32);
impl_rect_try_from!(u64, i8, i16, i32, i64);
impl_rect_try_from!(u128, i8, i16, i32, i64, i128);

// signed-to-unsigned
impl_rect_try_from!(i8, u8, u16, u32, u64, u128);
impl_rect_try_from!(i16, u16, u32, u64, u128);
impl_rect_try_from!(i32, u32, u64, u128);
impl_rect_try_from!(i64, u64, u128);
impl_rect_try_from!(i128, u128);
impl_rect_try_from!(i16, u8);
impl_rect_try_from!(i32, u16, u8);
impl_rect_try_from!(i64, u32, u16, u8);
impl_rect_try_from!(i128, u64, u32, u16, u8);

// usize/isize
impl_rect_try_from!(usize, isize);
impl_rect_try_from!(isize, usize);

#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::*;

    impl_rect_try_from!(usize, u8);
    impl_rect_try_from!(usize, u16, u32, u64, u128);
    impl_rect_try_from!(usize, i8, i16);
    impl_rect_try_from!(usize, i32, i64, i128);

    impl_rect_try_from!(isize, u8);
    impl_rect_try_from!(isize, u16, u32, u64, u128);
    impl_rect_try_from!(isize, i8);
    impl_rect_try_from!(isize, i16, i32, i64, i128);

    rect_rev!(impl_rect_try_from, usize, u32, u64, u128);
    rect_rev!(impl_rect_try_from, usize, i8, i16);
    rect_rev!(impl_rect_try_from, usize, i32, i64, i128);

    rect_rev!(impl_rect_try_from, isize, u16, u32, u64, u128);
    rect_rev!(impl_rect_try_from, isize, i32, i64, i128);
}

#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::*;

    impl_rect_try_from!(usize, u8, u16);
    impl_rect_try_from!(usize, u32, u64, u128);
    impl_rect_try_from!(usize, i8, i16, i32);
    impl_rect_try_from!(usize, i64, i128);

    impl_rect_try_from!(isize, u8, u16);
    impl_rect_try_from!(isize, u32, u64, u128);
    impl_rect_try_from!(isize, i8, i16);
    impl_rect_try_from!(isize, i32, i64, i128);

    rect_rev!(impl_rect_try_from, usize, u32);
    rect_rev!(impl_rect_try_from, usize, u64, u128);
    rect_rev!(impl_rect_try_from, usize, i8, i16, i32);
    rect_rev!(impl_rect_try_from, usize, i64, i128);

    rect_rev!(impl_rect_try_from, isize, u16);
    rect_rev!(impl_rect_try_from, isize, u32, u64, u128);
    rect_rev!(impl_rect_try_from, isize, i32);
    rect_rev!(impl_rect_try_from, isize, i64, i128);
}

#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    use super::*;

    impl_rect_try_from!(usize, u8, u16, u32);
    impl_rect_try_from!(usize, u64, u128);
    impl_rect_try_from!(usize, i8, i16, i32, i64);
    impl_rect_try_from!(usize, i128);

    impl_rect_try_from!(isize, u8, u16, u32);
    impl_rect_try_from!(isize, u64, u128);
    impl_rect_try_from!(isize, i8, i16, i32);
    impl_rect_try_from!(isize, i64, i128);

    rect_rev!(impl_rect_try_from, usize, u32, u64);
    rect_rev!(impl_rect_try_from, usize, u128);
    rect_rev!(impl_rect_try_from, usize, i8, i16, i32, i64);
    rect_rev!(impl_rect_try_from, usize, i128);

    rect_rev!(impl_rect_try_from, isize, u16, u32);
    rect_rev!(impl_rect_try_from, isize, u64, u128);
    rect_rev!(impl_rect_try_from, isize, i32, i64);
    rect_rev!(impl_rect_try_from, isize, i128);
}

impl<T> Rect<T> 
    where T: Number,
{
    /// Includes extreme points.
    pub fn is_line_segment_inside(&self, position: Vec2<T>, position2: Vec2<T>) -> bool 
    {
        position.x >= self.p1.x
        && position.x <= self.p2.x
        && position.y >= self.p1.y
        && position.y <= self.p2.y
        
        && position2.x >= self.p1.x
        && position2.x <= self.p2.x
        && position2.y >= self.p1.y
        && position2.y <= self.p2.y
    }

    /// Includes extreme points.
    /// # Panics:   
    /// //If the rect size lower than 2.
    pub fn is_crossing_by_line(&self, point: Vec2<T>, point2: Vec2<T>) -> Option<(Vec2<f64>, Vec2<f64>)> 
        where T: Into<f64>,
              Vec2<T>: FromIterator<<T as Add>::Output> + FromIterator<<T as Sub>::Output> + Into<Vec2<f64>>,
    {
        let mut statement = false;

        let mut p1: Vec2<f64> = point.into();
        let mut p2: Vec2<f64> = point2.into();

        let left_top_corner = self.p1;
        let right_top_corner = Vec2::new(self.p2.x, self.p1.y);
        let right_bottom_corner = self.p2;
        let left_bottom_corner = Vec2::new(self.p1.x, self.p2.y);

        //assert!(right_bottom_corner - left_top_corner >= Vec2::from(T::TWO), "invalid rect");
        let box_rect = Rect::new(left_top_corner, right_bottom_corner).into();

        let is_inside = point.is_inside_box(box_rect);
        let is_inside2 = point2.is_inside_box(box_rect);

        let left_top_corner = left_top_corner.into();
        let right_top_corner = right_top_corner.into();
        let right_bottom_corner = right_bottom_corner.into();
        let left_bottom_corner = left_bottom_corner.into();

        match line::cross_line_segments(
            p1,
            p2,
            left_top_corner,
            right_top_corner,
        ) {
            CrossLineResult::NoCrossing => (),
            CrossLineResult::Parallel => (),
            CrossLineResult::Matching => (),
            CrossLineResult::Crossing(clipping_point) => 
            {
                statement = true;

                // dbg!(clipping_point);
                // dbg!(is_inside, is_inside2);

                // if !is_inside && !is_inside2 {
                //     if (p1 - clipping_point).sqr_length() <= (p2 - clipping_point).sqr_length() {
                //         p1 = clipping_point;
                //     }
                //     else {
                //         p2 = clipping_point;
                //     }
                // }
                // else if !is_inside {
                //     if left_top_corner.x <= p2.x && p2.x <= right_top_corner.x && p2.y == left_top_corner.y {

                //     }

                //     p1 = clipping_point;
                // }
                // else if !is_inside2
                //     && !(left_top_corner.x <= p1.x && p1.x <= right_top_corner.x && p1.y == left_top_corner.y)
                // {
                //     p2 = clipping_point;
                // }

                if p1.y < p2.y {
                    p1 = clipping_point;
                }
                else {
                    p2 = clipping_point;
                }
            }
        }
        
        match line::cross_line_segments(
            p1, 
            p2, 
            right_top_corner, 
            right_bottom_corner,
        ) {
            CrossLineResult::NoCrossing => (),
            CrossLineResult::Parallel => (),
            CrossLineResult::Matching => (),
            CrossLineResult::Crossing(clipping_point) => 
            {
                statement = true;

                // dbg!(clipping_point);
                // dbg!(is_inside, is_inside2);

                // if !is_inside && !is_inside2 {
                //     if (p1 - clipping_point).sqr_length() <= (p2 - clipping_point).sqr_length() {
                //         p1 = clipping_point;
                //     }
                //     else {
                //         p2 = clipping_point;
                //     }
                // }
                // else if !is_inside
                //     && !(right_top_corner.y <= p2.y && p2.y <= right_bottom_corner.y && p2.y == right_top_corner.x)
                // {
                //     p1 = clipping_point;
                // }
                // else if !is_inside2
                //     && !(right_top_corner.y <= p1.y && p1.y <= right_bottom_corner.y && p1.y == right_top_corner.x)
                // {
                //     p2 = clipping_point;
                // }

                if p1.x > p2.x {
                    p1 = clipping_point;
                }
                else {
                    p2 = clipping_point;
                }
            }
        }

        match line::cross_line_segments(
            p1, 
            p2, 
            right_bottom_corner, 
            left_bottom_corner,
        ) {
            CrossLineResult::NoCrossing => (),
            CrossLineResult::Parallel => (),
            CrossLineResult::Matching => (),
            CrossLineResult::Crossing(clipping_point) => 
            {
                statement = true;

                // dbg!(clipping_point);
                // dbg!(is_inside, is_inside2);

                // if !is_inside && !is_inside2 {
                //     if (p1 - clipping_point).sqr_length() <= (p2 - clipping_point).sqr_length() {
                //         p1 = clipping_point;
                //     }
                //     else {
                //         p2 = clipping_point;
                //     }
                // }
                // else if !is_inside
                //     && !(left_bottom_corner.x <= p2.x && p2.x <= right_bottom_corner.x && p2.y == left_bottom_corner.y)
                // {
                //     p1 = clipping_point;
                // }
                // else if !is_inside2
                //     && !(left_bottom_corner.x <= p1.x && p1.x <= right_bottom_corner.x && p1.y == left_bottom_corner.y)
                // {
                //     p2 = clipping_point;
                // }

                if p1.y > p2.y {
                    p1 = clipping_point;
                }
                else {
                    p2 = clipping_point;
                }
            }
        }

        match line::cross_line_segments(
            p1, 
            p2, 
            left_bottom_corner, 
            left_top_corner,
        ) {
            CrossLineResult::NoCrossing => (),
            CrossLineResult::Parallel => (),
            CrossLineResult::Matching => (),
            CrossLineResult::Crossing(clipping_point) => 
            {
                statement = true;

                // dbg!(clipping_point);
                // dbg!(is_inside, is_inside2);

                // if !is_inside && !is_inside2 {
                //     if (p1 - clipping_point).sqr_length() <= (p2 - clipping_point).sqr_length() {
                //         p1 = clipping_point;
                //     }
                //     else {
                //         p2 = clipping_point;
                //     }
                // }
                // else if !is_inside
                //     && !(left_top_corner.y <= p2.y && p2.y <= left_bottom_corner.y && p2.y == left_top_corner.x)
                // {
                //     p1 = clipping_point;
                // }
                // else if !is_inside2
                //     && !(left_top_corner.y <= p1.y && p1.y <= left_bottom_corner.y && p1.y == left_top_corner.x)
                // {
                //     p2 = clipping_point;
                // }

                if p1.x < p2.x {
                    p1 = clipping_point;
                }
                else {
                    p2 = clipping_point;
                }
            }
        }

        if statement {
            Some((p1, p2))
        }
        else {
            None
        }
    }
}

impl<T> Rect<T>
    where T: Number + Float,
{
    pub fn round(&self) -> Self {
        Self {
            p1: self.p1.round(),
            p2: self.p2.round(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_initialization() {
        let rect: Rect<i32> = Rect::new(Vec2::ZERO, Vec2::ZERO);
        assert_eq!(rect, Rect {
            p1: Vec2::ZERO,
            p2: Vec2::ZERO,
        });

        assert_eq!(rect, Rect::default());

        let rect: Rect<i32> = Rect::default();
        let rect: Rect<i64> = rect.into();
        let _: Rect<i32> = rect.try_into().unwrap();
    }

    #[test]
    fn rect_is_crossing_by_line() {
        let point = Vec2::new(0, -2);
        let point2 = Vec2::new(1, 2);

        let rect = Rect::new(Vec2::ZERO, Vec2::TWO);
        let (p1, p2) = rect.is_crossing_by_line(point, point2).unwrap();
        assert_eq!((p1, p2), (Vec2::new(0.5, 0.0), Vec2::new(1.0, 2.0)));
    }
}