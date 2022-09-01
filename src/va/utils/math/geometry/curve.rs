use std::iter::Chain;

use roots::Roots;

use crate::utils::math::vector::vector2::Vec2;
use super::axis::Axis2d;

pub fn get_quad_curve_point(
    p0: Vec2<f64>, 
    p1: Vec2<f64>, 
    p2: Vec2<f64>, 
    t: f64
) -> Vec2<f64>
{
    p1 + (1.0 - t).powi(2) * (p0 - p1) + t.powi(2) * (p2 - p1)
}

#[derive(Debug)]
pub struct QuadCurveIntPointsByAxisIter {
    p0: Vec2<i32>,
    p1: Vec2<i32>,
    p2: Vec2<i32>,

    axis: usize,

    i: i32,
    len: i32,

    a: i32,

    values: [Option<f64>; 2],
}

impl QuadCurveIntPointsByAxisIter {
    pub fn new(p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>, axis: Axis2d) -> Self {
        let axis = axis as usize;

        let i = [p0[axis], p1[axis], p2[axis]].into_iter().min().unwrap();
        let len = [p0[axis], p1[axis], p2[axis]].into_iter().max().unwrap() + 1;

        let a = p0[axis] - 2*p1[axis] + p2[axis];

        let values = [None, None];

        Self {
            p0,
            p1,
            p2,
            axis,
            i,
            len,
            a,
            values,
        }
    }

    fn get_bezier_point_by_axis(&self, t: f64) -> f64 {
        let axis = (self.axis + 1) % 2;
        
        self.p1[axis] as f64 
            + (1.0 - t).powi(2) * (self.p0[axis] - self.p1[axis]) as f64 
            + t.powi(2) * (self.p2[axis] - self.p1[axis]) as f64
    }

    fn get_curve_point_by_axis(&mut self) {
        let idx = self.axis;

        if self.a == 0 {
            if self.p1[idx] - self.p0[idx] == 0 {
                return;
            }

            let t = (self.i - self.p0[idx]) as f64 / (2.0 * (self.p1[idx] - self.p0[idx]) as f64);

            if t < 0.0 || t > 1.0 {
                return;
            }

            let j = self.get_bezier_point_by_axis(t);
            self.values[0] = Some(j);
            return;
        }

        let d = self.p1[idx].pow(2) - self.p0[idx] * self.p2[idx] + self.i * (-2*self.p1[idx] + self.p0[idx] + self.p2[idx]);
        if d < 0 {
            return;
        }

        let a = self.a as f64;
        if d == 0 {
            let t = (self.p0[idx] - self.p1[idx]) as f64 / a;

            if t < 0.0 || t > 1.0 {
                return;
            }

            let j = self.get_bezier_point_by_axis(t);
            self.values[0] = Some(j);
        }
        else {
            let sqrt_d = (d as f64).sqrt();

            let t1 = ((self.p0[idx] - self.p1[idx]) as f64 + sqrt_d) / a;
            let t2 = ((self.p0[idx] - self.p1[idx]) as f64 - sqrt_d) / a;

            let curve1 = if t1 >= 0.0 && t1 <= 1.0 {
                let j = self.get_bezier_point_by_axis(t1);
                Some(j)
            }
            else {
                None
            };
            
            let curve2= if t2 >= 0.0 && t2 <= 1.0 {
                let j = self.get_bezier_point_by_axis(t2);
                Some(j)
            }
            else {
                None
            };
            
            self.values = [curve1, curve2];
        }
    }
}

impl Iterator for QuadCurveIntPointsByAxisIter {
    type Item = Vec2<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        'iter_loop: loop {
            let idx = self.axis;
            let idx_2 = (self.axis + 1) % 2;

            for element in &mut self.values {
                if let Some(val) = element.take() {
                    let mut new_point: Vec2<f64> = Vec2::default();
                    new_point[idx] = (self.i - 1).into(); // getting previous
                    new_point[idx_2] = val;

                    break 'iter_loop Some(new_point);
                }
            }

            if self.i >= self.len {
                break None;
            }
    
            self.get_curve_point_by_axis();
            self.i += 1;
        }
    }
}

#[derive(Debug)]
pub struct QuadCurveIntPointsIter {
    inner: Chain<QuadCurveIntPointsByAxisIter, QuadCurveIntPointsByAxisIter>,
}

impl QuadCurveIntPointsIter {
    pub fn new(
        p0: Vec2<i32>,
        p1: Vec2<i32>,
        p2: Vec2<i32>,
    ) -> Self
    {
        let x_iter = QuadCurveIntPointsByAxisIter::new(p0, p1, p2, Axis2d::X);
        let y_iter = QuadCurveIntPointsByAxisIter::new(p0, p1, p2, Axis2d::Y);
    
        Self {
            inner: x_iter.chain(y_iter),
        }
    }
}

impl Iterator for QuadCurveIntPointsIter {
    type Item = Vec2<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html
pub fn subdivide_quad_curve(
    p0: Vec2<f64>,
    p1: Vec2<f64>,
    p2: Vec2<f64>,
) -> Vec<f64>
{
    const THRESHOLD: f64 = 0.1;//10.0;//0.1;//10.0;//0.5;
        
    // Map quadratic bezier segment to y = x^2 parabola
    let (x0, x2, scale, cross) = map_to_basic(p0, p1, p2);
    // Compute approximate integral for x at both endpoints
    let a0 = approx_myint(x0);
    let a2 = approx_myint(x2);
    let count = 0.5 * (a2 - a0).abs() * (scale / THRESHOLD).sqrt();
    let n = count.ceil();

    let approx_x0 = approx_inv_myint(a0);
    let approx_x2 = approx_inv_myint(a2);

    let mut result = vec![0.0];

    // Subdivide evenly and compute approximate inverse integral.
    for i in 1..n as u64 {
        let x = approx_inv_myint(a0 + ((a2 - a0) * i as f64) / n);

        // Map x parameter back to t parameter for the original segment.
        let t = (x - approx_x0) / (approx_x2 - approx_x0);
        result.push(t);
    }
    
    result.push(1.0);
    result
}

fn map_to_basic(p0: Vec2<f64>, p1: Vec2<f64>, p2: Vec2<f64>) -> (f64, f64, f64, f64) {
    let ddx = 2.0 * p1.x - p0.x - p2.x;
    let ddy = 2.0 * p1.y - p0.y - p2.y;
    let u0 = (p1.x - p0.x) * ddx + (p1.y - p0.y) * ddy;
    let u2 = (p2.x - p1.x) * ddx + (p2.y - p1.y) * ddy;

    let cross = (p2.x - p0.x) * ddy - (p2.y - p0.y) * ddx;

    let x0 = u0 / cross;
    let x2 = u2 / cross;

    let scale = cross.abs() / (ddx.hypot(ddy) * (x2 - x0).abs());

    (x0, x2, scale, cross)
}

fn approx_myint(x: f64) -> f64 { 
    const D: f64 = 0.67; 
    x / (1.0 - D + (D.powi(4) + 0.25 * x * x).powf(0.25))
}

fn approx_inv_myint(x: f64) -> f64 { 
    const B: f64 = 0.39; 
    x * (1.0 - B + (B * B + 0.25 * x * x).sqrt())
}

// // https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html
// #[derive(Debug)]
// pub struct QuadCurvePointsIter {
//     p0: Vec2<i32>,
//     p1: Vec2<i32>,
//     p2: Vec2<i32>,
//     n: usize,
//     count: usize,
// }

// // https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html
// impl QuadCurvePointsIter {
//     pub fn new(
//         p0: Vec2<i32>,
//         p1: Vec2<i32>,
//         p2: Vec2<i32>,
//     ) -> Self
//     {
//         const THRESH: f64 = 0.5;
        
//         // Map quadratic bezier segment to y = x^2 parabola
//         let (x0, x2, scale, cross) = QuadCurvePointsIter::map_to_basic(p0, p1, p2);

//         // Compute approximate integral for x at both endpoints
//         let a0 = QuadCurvePointsIter::approx_myint(x0);
//         let a2 = QuadCurvePointsIter::approx_myint(x2);
//         let count = 0.5 * (a2 - a0).abs() * (scale / THRESH).sqrt();
//         let n = count.ceil();

//         let approx_x0 = QuadCurvePointsIter::approx_inv_myint(a0);
//         let approx_x2 = QuadCurvePointsIter::approx_inv_myint(a2);

//         let mut result = vec![0.0];

//         // Subdivide evenly and compute approximate inverse integral.
//         for i in 1..n {
//             let x = QuadCurvePointsIter::approx_inv_myint(a0 + ((a2 - a0) * i) / n);

//             // Map x parameter back to t parameter for the original segment.
//             let t = (x - approx_x0) / (approx_x2 - approx_x0);
//             result.push(t);
//         }
        
//         result.push(1.0);

//         Self { 
//             p0, 
//             p1, 
//             p2, 
//             n: 0,
//             count: ,
//         }
//     }

//     fn map_to_basic(p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>) -> (f64, f64, f64, i32) {
//         let ddx = 2 * p1.x - p0.x - p2.x;
//         let ddy = 2 * p1.y - p0.y - p2.y;
//         let u0: f64 = ((p1.x - p0.x) * ddx + (p1.y - p0.y) * ddy).into();
//         let u2: f64 = ((p2.x - p1.x) * ddx + (p2.y - p1.y) * ddy).into();

//         let cross = (p2.x - p0.x) * ddy - (p2.y - p0.y) * ddx;
//         let fcross: f64 = cross.into();

//         let x0: f64 = u0 / fcross;
//         let x2: f64 = u2 / fcross;

//         let ddx: f64 = ddx.into();
//         let ddy: f64 = ddy.into();

//         let scale: f64 = fcross.abs() / (ddx.hypot(ddy) * (x2 - x0).abs());

//         (x0, x2, scale, cross)
//     }

//     fn approx_myint(x: f64) -> f64 { 
//         const D: f64 = 0.67; 
//         x / (1.0 - D + (D.powi(4) + 0.25 * x * x).powf(0.25))
//     }

//     fn approx_inv_myint(x: f64) -> f64 { 
//         const B: f64 = 0.39; 
//         x * (1.0 - B + (B * B + 0.25 * x * x).sqrt())
//     }
// }

// impl Iterator for QuadCurvePointsIter {
//     type Item = Vec2<f64>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[derive(Debug)]
pub struct CubeCurveIntPointsByAxisIter {
    p0: Vec2<i32>,
    p1: Vec2<i32>,
    p2: Vec2<i32>,
    p3: Vec2<i32>,

    axis: usize,

    i: i32,
    len: i32,

    a3: i32,
    a2: i32,
    a1: i32,

    values: [Option<f64>; 3],
}

impl CubeCurveIntPointsByAxisIter {
    pub fn new(p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>, p3: Vec2<i32>, axis: Axis2d) -> Self {
        let axis = axis as usize;

        let i = *[p0[axis], p1[axis], p2[axis], p3[axis]].iter().min().unwrap();
        let len = [p0[axis], p1[axis], p2[axis], p3[axis]].iter().max().unwrap() + 1;

        let a3 = -p0[axis] + 3*p1[axis] - 3*p2[axis] + p3[axis];
        let a2 = 3*p0[axis] - 6*p1[axis] + 3*p2[axis];
        let a1 = -3*p0[axis] + 3*p1[axis];

        let values = [None; 3];

        Self {
            p0,
            p1,
            p2,
            p3,
            axis,
            i,
            len,
            a3,
            a2,
            a1,
            values,
        }
    }

    fn get_bezier_point_by_axis(&self, t: f64) -> f64 {
        let axis = (self.axis + 1) % 2;

        (1.0 - t).powi(3) * self.p0[axis] as f64
        + 3.0 * t * (1.0 - t).powi(2) * self.p1[axis] as f64
        + 3.0 * t.powi(2) * (1.0 - t) * self.p2[axis] as f64
        + t.powi(3) * self.p3[axis] as f64
    }

    fn get_curve_point_by_axis(&mut self) {
        let idx = self.axis;
        let a0 = self.p0[idx] - self.i;

        let result = roots::find_roots_cubic(self.a3 as f64, self.a2 as f64, self.a1 as f64, a0 as f64);
        match result {
            Roots::One(val) => {
                if 0.0 <= val[0] && val[0] <= 1.0 {
                    self.values[0] = Some(self.get_bezier_point_by_axis(val[0]));
                }
            }
            Roots::Two(val) => {
                for i in 0..val.len() {
                    if 0.0 <= val[i] && val[i] <= 1.0 {
                        self.values[i] = Some(self.get_bezier_point_by_axis(val[i]));
                    }
                }
            }
            Roots::Three(val) => {
                for i in 0..val.len() {
                    if 0.0 <= val[i] && val[i] <= 1.0 {
                        self.values[i] = Some(self.get_bezier_point_by_axis(val[i]));
                    }
                }
            }
            _ => (),
        }
    }
}

impl Iterator for CubeCurveIntPointsByAxisIter {
    type Item = Vec2<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        'iter_loop: loop {
            let idx = self.axis;
            let idx_2 = (self.axis + 1) % 2;

            for element in &mut self.values {
                if let Some(val) = element.take() {
                    let mut new_point = Vec2::default();
                    new_point[idx] = (self.i - 1).into(); // getting previous
                    new_point[idx_2] = val;
                    
                    break 'iter_loop Some(new_point);
                }
            }

            if self.i >= self.len {
                break None;
            }
    
            self.get_curve_point_by_axis();
            self.i += 1;
        }
    }
}

#[derive(Debug)]
pub struct CubeCurveIntPointsIter {
    inner: Chain<CubeCurveIntPointsByAxisIter, CubeCurveIntPointsByAxisIter>,
}

impl CubeCurveIntPointsIter {
    pub fn new(
        p0: Vec2<i32>,
        p1: Vec2<i32>,
        p2: Vec2<i32>,
        p3: Vec2<i32>,
    ) -> Self
    {
        let x_iter = CubeCurveIntPointsByAxisIter::new(p0, p1, p2, p3, Axis2d::X);
        let y_iter = CubeCurveIntPointsByAxisIter::new(p0, p1, p2, p3, Axis2d::Y);
    
        Self {
            inner: x_iter.chain(y_iter),
        }
    }
}

impl Iterator for CubeCurveIntPointsIter {
    type Item = Vec2<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}