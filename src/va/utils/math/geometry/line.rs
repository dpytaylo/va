use std::borrow::Borrow;
// abcdefghijklmnopqrstuvwxyz
use std::fmt::Debug;
use std::iter::{Map, self, Take, Repeat, Zip};
use std::mem::replace;
use std::ops::{Sub, Mul};

use crate::utils::math::is_equal::IsCopyTypeEqual;
use crate::utils::{math::vector::vector2::Vec2, number::Number, cast::Cast};
use super::axis::Axis2d;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Line<T> 
    where T: Debug + Default + Clone,
{
    pub p1: T,
    pub p2: T,
}

impl<T> Line<T> 
    where T: Debug + Default + Clone,
{
    pub fn new(p1: T, p2: T) -> Self {
        Self {
            p1, p2
        }
    }
}

pub type Line2<T> = Line<Vec2<T>>;

#[derive(Debug)]
pub struct LinePointsByAxisIter {
    i: usize,
    j: usize,

    xy_length: Vec2<i32>,

    error: i32,
    delta_error: i32,
    j_value: i32,
    dir: i32,

    i_value: i32,
    end_i_value: i32,
}

impl LinePointsByAxisIter {
    pub fn new(
        point: Vec2<i32>, 
        point2: Vec2<i32>,
        axis: Axis2d,
    ) -> Self 
    {
        let i = axis as usize;
        let j = (i + 1) % 2;
    
        let (point, point2) = if point2[i] > point[i] {
            (point, point2)
        }
        else {
            (point2, point)
        };    
    
        let xy_length = (point - point2).abs() + 1;
    
        let error = 0;
        let delta_error = xy_length[j];
        let j_value = point[j];
        let dir = if point[j] < point2[j] {1} else {-1};
    
        let i_value = point[i];
        let end_i_value = point2[i];
    
        Self { 
            i, 
            j, 
            xy_length, 
            error, 
            delta_error, 
            j_value, 
            dir, 
            i_value, 
            end_i_value,
        }
    }
}

impl Iterator for LinePointsByAxisIter {
    type Item = Vec2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i_value > self.end_i_value {
            return None;
        }

        let mut pos = Vec2::default();
        pos[self.i] = self.i_value;
        pos[self.j] = self.j_value;
        
        self.error += self.delta_error;
        let k = if self.xy_length[self.i] == 0 {
            0
        }
        else { 
            self.error / self.xy_length[self.i]
        };
        self.error %= self.xy_length[self.i];

        self.j_value += k * self.dir;
        self.i_value += 1;

        Some(pos)
    }
}

impl ExactSizeIterator for LinePointsByAxisIter {
    fn len(&self) -> usize {
        (self.end_i_value - self.i_value) as usize + 1
    }
}

#[derive(Debug)]
pub struct LinePointsIter {
    //x_iter: LinePointsByAxisIter,
    //y_iter: LinePointsByAxisIter,
}

impl LinePointsIter {
    pub fn new(point: Vec2<i32>, point2: Vec2<i32>) -> LinePointsByAxisIter {
        //let x_iter = LinePointsByAxisIter::new(point, point2, Axis2d::X);
        //let y_iter = LinePointsByAxisIter::new(point, point2, Axis2d::Y);

        let dif = (point2 - point).abs();
        if dif.x >= dif.y {
            LinePointsByAxisIter::new(point, point2, Axis2d::X)
        }
        else {
            LinePointsByAxisIter::new(point, point2, Axis2d::Y)
        }
    }
}

// impl Iterator for LinePointsIter {
//     type Item = Vec2<i32>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.x_iter.next() {
//             Some(val) => Some(val),
//             None => {
//                 self.y_iter.next()
//             }
//         }
//     }
// }

// impl ExactSizeIterator for LinePointsIter {
//     fn len(&self) -> usize {
//         self.x_iter.len() + self.y_iter.len()
//     }
// }

// #[derive(Debug)]
// pub struct WuLinePointsByAxisIter {
//     i: usize,
//     j: usize,

//     xy_length: Vec2<i32>,

//     k: f32,

//     i_value: i32,
//     end_i_value: i32,
// }

// impl WuLinePointsByAxisIter {
//     pub fn new(
//         point: Vec2<i32>, 
//         point2: Vec2<i32>,
//         axis: Axis2d,
//     ) -> Self 
//     {
//         let i = axis as usize;
//         let j = (i + 1) % 2;
    
//         let (point, point2) = if point2[i] > point[i] {
//             (point, point2)
//         }
//         else {
//             (point2, point)
//         };    
    
//         let xy_length = (point - point2).abs() + 1;
//         let k = xy_length[j] as f32 / xy_length[i] as f32;
    
//         let i_value = point[i];
//         let end_i_value = point2[i];
    
//         Self { 
//             i, 
//             j, 
//             xy_length, 
//             k,
//             i_value, 
//             end_i_value,
//         }
//     }
// }

// impl Iterator for WuLinePointsByAxisIter {
//     type Item = Vec2<i32>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i_value > self.end_i_value {
//             return None;
//         }

//         let mut pos = Vec2::default();
//         pos[self.i] = self.i_value;
//         pos[self.j] = self.j_value;
        
//         self.error += self.delta_error;
//         let k = self.error / self.xy_length[self.i];
//         self.error %= self.xy_length[self.i];

//         self.j_value += k * self.dir;
//         self.i_value += 1;

//         Some(pos)
//     }
// }

// impl ExactSizeIterator for WuLinePointsByAxisIter {
//     fn len(&self) -> usize {
//         (self.end_i_value - self.i_value) as usize + 1
//     }
// }

pub fn is_crossing_line_segments(
    line_point: Vec2<i32>,
    line_point2: Vec2<i32>,
    line2_point: Vec2<i32>,
    line2_point2: Vec2<i32>,
) -> bool
{
    let (x1, y1) = line_point.into();
    let (x2, y2) = line_point2.into();
    let (x3, y3) = line2_point.into();
    let (x4, y4) = line2_point2.into();

    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

    if denominator == 0 {
        return false;
    }

    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) as f64 / denominator as f64;
    let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) as f64 / denominator as f64;

    (0.0 <= ua && ua <= 1.0) && (0.0 <= ub && ub <= 1.0)
}

#[derive(Debug)]
pub enum CrossLineResult {
    NoCrossing,
    Parallel,
    Matching,
    Crossing(Vec2<f64>),
}

// http://algolist.ru/maths/geom/intersect/lineline2d.php
pub fn cross_line_segments(
    line_point: Vec2<f64>, 
    line_point2: Vec2<f64>,
    line2_point: Vec2<f64>,
    line2_point2: Vec2<f64>,
) -> CrossLineResult
{
    let (x1, y1) = line_point.into();
    let (x2, y2) = line_point2.into();
    let (x3, y3) = line2_point.into();
    let (x4, y4) = line2_point2.into();

    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
 
    let a = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let b = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

    if denominator.is_equal(0.0, 0.0001) {
        if a.is_equal(0.0, 0.0001) || b.is_equal(0.0, 0.0001) {
            return CrossLineResult::Matching;
        }
        
        return CrossLineResult::Parallel;
    }

    let ua = a / denominator;
    let ub = b / denominator;

    if !((0.0 <= ua && ua <= 1.0) && (0.0 <= ub && ub <= 1.0)) {
        return CrossLineResult::NoCrossing;
    }

    CrossLineResult::Crossing(
        Vec2::new(
            x1 + ua * (x2 - x1),
            y1 + ua * (y2 - y1),
        ),
    )
}

/// Not contains separated points
pub fn intersect<T>(line: Line<i32>, mut iter: T) -> Vec<Line<i32>> 
    where T: Iterator<Item = i32>,
{
    let mut buffer = vec![Line::new(line.p1, line.p2)];

    let mut first_x = match iter.next() {
        Some(val) => val,
        None => return buffer,
    };
    let mut last_x = first_x;

    let mut func = |first_x, last_x| {
        let mut remove_idx = Vec::new();
        let mut add_element = Vec::new();

        for i in 0..buffer.len() {
            /*
                ***|***|*** 
            */
            if first_x <= buffer[i].p1 
                && last_x >= buffer[i].p2
            {
                remove_idx.push(i);
            }

            /*
                ***|***---|
            */
            else if first_x <= buffer[i].p1
                && last_x >= buffer[i].p1
                && last_x < buffer[i].p2
            {
                buffer[i].p1 = last_x + 1;
            }

            /*
                |---***|***
            */
            else if first_x > buffer[i].p1
                && first_x <= buffer[i].p2
                && last_x >= buffer[i].p2
            {
                buffer[i].p2 = first_x - 1;
            }

            /*
                |---***---|
            */
            else if first_x > buffer[i].p1
                && first_x < buffer[i].p2
                && last_x > buffer[i].p1
                && last_x < buffer[i].p2
            {
                add_element.push(Line::new(last_x + 1, replace(&mut buffer[i].p2, first_x - 1)));
            }
        }

        for index in remove_idx.into_iter().rev() {
            buffer.remove(index);
        }

        for element in add_element {
            buffer.push(element);
        }
    };

    for point in iter {
        if point - last_x > 1 {
            func(first_x, last_x);
            first_x = point;
        }

        last_x = point;
    }

    func(first_x, last_x);
    buffer
}

/*
pub fn intersect_includes<T>(line: Line<i32>, mut iter: T) -> Vec<Line<i32>> 
    where T: Iterator<Item = i32>,
{
    let mut buffer = vec![Line::new(line.p1, line.p2)];

    let mut first_x = match iter.next() {
        Some(val) => val,
        None => return buffer,
    };
    let mut last_x = first_x;

    let mut func = |first_x, last_x| {
        let mut remove_idx = Vec::new();
        let mut add_element = Vec::new();

        for i in 0..buffer.len() {
            /*
                ***|***|*** 
            */
            if first_x <= buffer[i].p1 
                && last_x >= buffer[i].p2
            {
                remove_idx.push(i);
            }

            /*
                ***|***---|
            */
            else if first_x <= buffer[i].p1
                && last_x >= buffer[i].p1
                && last_x < buffer[i].p2
            {
                buffer[i].p1 = last_x + 1;
            }

            /*
                |---***|***
            */
            else if first_x > buffer[i].p1
                && first_x <= buffer[i].p2
                && last_x >= buffer[i].p2
            {
                buffer[i].p2 = first_x - 1;
            }

            /*
                |---***---|
            */
            else if first_x > buffer[i].p1
                && first_x < buffer[i].p2
                && last_x > buffer[i].p1
                && last_x < buffer[i].p2
            {
                add_element.push(Line::new(last_x + 1, replace(&mut buffer[i].p2, first_x - 1)));
            }
        }

        for index in remove_idx.into_iter().rev() {
            buffer.remove(index);
        }

        for element in add_element {
            buffer.push(element);
        }
    };

    for point in iter {
        if point - last_x > 1 {
            func(first_x, last_x);
            first_x = point;
        }

        last_x = point;
    }

    func(first_x, last_x);
    buffer
}
*/

pub struct LineEquation {
    pub axis: Axis2d,
    pub k: f64,
    pub b: f64,
}

impl LineEquation {
    pub fn new<T>(p1: Vec2<T>, p2: Vec2<T>) -> LineEquation
        where T: Number,
              Vec2<T>: FromIterator<<T as std::ops::Sub>::Output> + Into<Vec2<f64>>,
    {
        let delta = p1 - p2;
        let p1 = p1.into();
        let p2 = p2.into();

        if delta.x.abs() >= delta.y.abs() {
            let delta: Vec2<f64> = delta.into();
            let k = if delta.x.is_equal(0.0, f64::EPSILON) {
                0.0
            }
            else {
                delta.y / delta.x
            };

            let b = p1.y - k * p1.x;

            LineEquation {
                axis: Axis2d::X,
                k,
                b,
            }
        }
        else {
            let delta: Vec2<f64> = delta.into();
            let k = if delta.y.is_equal(0.0, f64::EPSILON) {
                0.0
            }
            else {
                delta.x / delta.y
            };

            let b = p1.x - k * p1.y;

            LineEquation {
                axis: Axis2d::Y,
                k,
                b,
            }
        }
    }

    pub fn get_y_by_x(&self, x: f64) -> f64 {
        match self.axis {
            Axis2d::X => self.k * x + self.b,
            Axis2d::Y => {
                if self.k.is_equal(0.0, f64::EPSILON) {
                    self.b
                }
                else {
                    (x - self.b) / self.k
                }
            }
        }
    }

    pub fn get_x_by_y(&self, y: f64) -> f64 {
        match self.axis {
            Axis2d::X => {
                if self.k.is_equal(0.0, f64::EPSILON) {
                    self.b
                }
                else {
                    (y - self.b) / self.k
                }
            }
            Axis2d::Y => self.k * y + self.b,
        }
    }

    pub fn iter_get_y_by_x<T, U>(&self, iter: T) -> Map<Zip<T, Take<Repeat<(f64, f64)>>>, fn((U, (f64, f64))) -> f64>
        where T: Iterator<Item = U> + ExactSizeIterator,
              U: Borrow<f64>,
    {
        let length = iter.len();
        let iter = iter.zip(iter::repeat((self.k, self.b)).take(length));

        match self.axis {
            Axis2d::X => iter.map(|(x, (k, b))| k * *x.borrow() + b),
            Axis2d::Y => {
                if self.k.is_equal(0.0, f64::EPSILON) {
                    iter.map(|(_, (_, b))| b)
                }
                else {
                    iter.map(|(x, (k, b))| (*x.borrow() - b) / k)
                }
            }
        }
    }

    pub fn iter_get_x_by_y<T, U>(&self, iter: T) -> Map<Zip<T, Take<Repeat<(f64, f64)>>>, fn((U, (f64, f64))) -> f64>
        where T: Iterator<Item = U> + ExactSizeIterator,
              U: Borrow<f64>,
    {
        let length = iter.len();
        let iter = iter.zip(iter::repeat((self.k, self.b)).take(length));

        match self.axis {
            Axis2d::X => {
                if self.k.is_equal(0.0, f64::EPSILON) {
                    iter.map(|(_, (_, b))| b)
                }
                else {
                    iter.map(|(y, (k, b))| (*y.borrow() - b) / k)
                }
            }
            Axis2d::Y => iter.map(|(y, (k, b))| k * *y.borrow() + b),
        }
    }

    // pub fn get_range_y_by_x<T, I>(&self, iter: T) -> Map<T, &dyn Fn(I) -> f64>
    //     where T: Iterator<Item = I>,
    //           I: Borrow<f64>,
    // {
    //     match self.axis {
    //         Axis2d::X => iter.map(&|x| self.k * *x.borrow() + self.b),
    //         Axis2d::Y => {
    //             if self.k.is_equal(0.0, f64::EPSILON) {
    //                 iter.map(&|_| self.b)
    //             }
    //             else {
    //                 iter.map(&|x| (*x.borrow() - self.b) / self.k)
    //             }
    //         }
    //     }
    // }

    // pub fn get_range_x_by_y<T, I>(&self, iter: T) -> Map<T, &dyn Fn(I) -> f64>
    //     where T: Iterator<Item = I>,
    //           I: Borrow<f64>,
    // {
    //     match self.axis {
    //         Axis2d::X => {
    //             if self.k.is_equal(0.0, f64::EPSILON) {
    //                 iter.map(&|_| self.b)
    //             }
    //             else {
    //                 iter.map(&|y| (*y.borrow() - self.b) / self.k)
    //             }
    //         }
    //         Axis2d::Y => iter.map(&|y| self.k * *y.borrow() + self.b),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn test_cross_line_segments() {
        let val = match cross_line_segments(
            Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), 
            Vec2::new(2.0, 2.0), Vec2::new(8.0, -2.0),
        )
        {
            CrossLineResult::Crossing(val) => val,
            _ => panic!("invalid result"),
        };

        assert_eq!(val, Vec2::new(5.0, 0.0));

        let val = match cross_line_segments(
            Vec2::new(137.0, 0.0), Vec2::new(141.0, -1.0), 
            Vec2::new(0.0, 0.0), Vec2::new(300.0, 0.0),
        )
        {
            CrossLineResult::Crossing(val) => val,
            _ => panic!("invalid result"),
        };

        assert_eq!(val, Vec2::new(137.0, 0.0));

        let rp1 = Vec2::from(0.0);
        let rp2 = Vec2::from(1199.0);

        let p1 = Vec2::new(38.0, 0.0);
        let p2 = Vec2::new(33.0, -14.0);

        let val = match cross_line_segments(
            p1, p2, 
            rp1, Vec2::new(rp2.x, rp1.y),
        )
        {
            CrossLineResult::Crossing(val) => val,
            _ => panic!("invalid result"),
        };

        assert_eq!(val, Vec2::new(38.0, 0.0));
    }

    #[test]
    fn test_intersect() {
        let line = Line::new(10, 20);
        let points = [1, 4, 9, 10, 11, 15, 17, 18, 20, 21, 22];
        let result = [Line::new(12, 14), Line::new(16, 16), Line::new(19, 19)];

        let r = intersect(line, points.into_iter());

        assert_eq!(r.len(), result.len());
        assert_eq!(r.into_iter().zip(result.into_iter()).find(|(l1, l2)| l1 != l2), None);
    }

    #[test]
    fn test_line_equation() {
        let equation = LineEquation::new(Vec2::new(-2, -2), Vec2::new(6, 2));
        assert!(equation.get_x_by_y(2.0).is_equal(6.0, EPSILON));
        assert!(equation.get_y_by_x(2.0).is_equal(0.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(-3, 2), Vec2::new(3, 2));
        assert!(equation.get_y_by_x(1.0).is_equal(2.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(-2, -6), Vec2::new(2, 6));
        assert!(equation.get_y_by_x(1.0).is_equal(3.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(0, 0), Vec2::new(4, 2));
        assert!(equation.iter_get_y_by_x([2.0].into_iter()).next().unwrap().is_equal(1.0, EPSILON));
        assert!(equation.iter_get_x_by_y([1.0].into_iter()).next().unwrap().is_equal(2.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(0, 0), Vec2::new(4, -2));
        assert!(equation.iter_get_y_by_x([2.0].into_iter()).next().unwrap().is_equal(-1.0, EPSILON));
        assert!(equation.iter_get_x_by_y([-1.0].into_iter()).next().unwrap().is_equal(2.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(0, 0), Vec2::new(-4, -2));
        assert!(equation.iter_get_y_by_x([-2.0].into_iter()).next().unwrap().is_equal(-1.0, EPSILON));
        assert!(equation.iter_get_x_by_y([-1.0].into_iter()).next().unwrap().is_equal(-2.0, EPSILON));

        let equation = LineEquation::new(Vec2::new(0, 0), Vec2::new(-4, 2));
        assert!(equation.iter_get_y_by_x([-2.0].into_iter()).next().unwrap().is_equal(1.0, EPSILON));
        assert!(equation.iter_get_x_by_y([1.0].into_iter()).next().unwrap().is_equal(-2.0, EPSILON));
    }
}