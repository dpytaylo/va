// abcdefghijklmnopqrstuvwxyz
use std::cmp::{Ordering, min, max};
use std::collections::BTreeSet;
use std::f32::consts::FRAC_PI_2;
use std::iter::{Map, Zip};
use std::mem::{replace, self};
use std::rc::Rc;
use std::vec;

use rand::Rng;
use ttf_parser::OutlineBuilder;

use crate::graphics::buffer::buffer2d::save_buffer;
use crate::graphics::rasterizate::SimpleRasterizate;
use crate::manager::Manager;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::axis::{Axis, Axis2d};
use crate::utils::math::geometry::curve::{QuadCurveIntPointsIter, CubeCurveIntPointsIter, self};
use crate::utils::math::geometry::line::{self, Line, LinePointsIter, LinePointsByAxisIter, LineEquation};
use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::matrix::matrix3x3::Mat3x3;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;
use crate::utils::number::Number;

use super::buffer::buffer2d::{Buffer2d, Buffer2dRead, Buffer2dSlice, Buffer2dMutSlice};
use super::rasterizate::Rasterizate;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum SweepDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, PartialEq)]
enum Side {
    Above,
    Below,
    Both,
}

#[derive(Debug)]
struct SegmentInfo {
    pos: i32,
    side: Side,
}

impl SegmentInfo {
    fn new(pos: i32, side: Side) -> Self {
        Self { 
            pos,
            side,
        }
    }
}

impl PartialEq for SegmentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for SegmentInfo {}

impl PartialOrd for SegmentInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl Ord for SegmentInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct GlyphRender<'a> {
    buffer: &'a mut Buffer2d<Vec4<f32>>,
    transform: Mat3x3<f32>,

    points: Vec<Vec2<f32>>,
    prev_point: Vec2<f32>,
    outlines: Vec<(SweepDirection, Vec<Vec2<f32>>)>,
    was_closed: bool,
}

impl<'a> GlyphRender<'a> {
    pub fn new(buffer: &'a mut Buffer2d<Vec4<f32>>, transform: Mat3x3<f32>) -> Self {
        Self {
            buffer,
            transform,

            points: Vec::new(),
            prev_point: Vec2::default(),
            outlines: Vec::new(),
            was_closed: false,
        }
    }

    pub fn was_closed(&self) -> bool {
        self.was_closed
    }

    fn do_transform(&self, point: Vec2<f32>) -> Vec2<f32> {
        (self.transform * Vec3::new(point.x, point.y, 1.0)).xy()
    }

    pub fn push_outline(&mut self) {
        if self.points.len() < 2 {
            self.points.clear();
            return;
        }
        
        let mut direction_counter = 0.0;
        let mut prev_n = (self.points[1] - self.points[0]).normalize();

        let mut closure = |p1: Vec2<f32>, p2: Vec2<f32>| {
            let n = (p2 - p1).normalize();
            let z = prev_n.x * n.y - prev_n.y * n.x;

            let angle = prev_n.dot(n).clamp(-1.0, 1.0).acos();
            prev_n = n;

            // By clockwise
            if z >= 0.0 {
                direction_counter += angle;
            }
            else {
                direction_counter -= angle;
            }
        };

        for i in 1..self.points.len() - 1 {
            closure(self.points[i], self.points[i + 1]);
        }
        closure(*self.points.last().unwrap(), self.points[0]);

        let direction = if direction_counter >= 0.0 {
            SweepDirection::Clockwise
        }
        else {
            SweepDirection::Counterclockwise
        };
        
        self.outlines.push((direction, mem::replace(&mut self.points, Vec::new())));
    }

    pub fn rasterizate(&mut self) {
        for y in 0..self.buffer.height() as i32 {
            let mut cw_buffer = Vec::new();
            let mut cc_buffer = Vec::new();

            for (dir, points) in &self.outlines {
                let buffer = match dir {
                    SweepDirection::Clockwise => &mut cw_buffer,
                    SweepDirection::Counterclockwise => &mut cc_buffer,
                };

                let mut closure = |p1: Vec2<f32>, p2: Vec2<f32>| {
                    let p1: Vec2<i32> = p1.round().cast();
                    let p2: Vec2<i32> = p2.round().cast();

                    // Ignore horizontal segments
                    if p1.y == p2.y {
                        return;
                    }

                    let (min, max) = if p1.y <= p2.y {
                        (p1.y, p2.y)
                    }
                    else {
                        (p2.y, p1.y)
                    };
                    
                    if y < min || y > max {
                        return;
                    }

                    let side = if p1.y < y && p2.y > y 
                        || p1.y > y && p2.y < y 
                    {
                        Side::Both
                    }
                    else if p1.y >= y && p2.y >= y {
                        Side::Below // y coordinate down
                    }
                    else {
                        Side::Above
                    };

                    let equation = LineEquation::new(p1, p2);
                    buffer.push(SegmentInfo::new(equation.get_x_by_y(y.into()).round().cast(), side));
                };

                for i in 1..points.len() {
                    closure(points[i - 1], points[i]);
                }

                closure(*points.last().unwrap(), points[0]);
            }
            
            cw_buffer.sort_unstable();
            
            let mut remove_idx = Vec::new();
            for i in 1..cw_buffer.len() {
                if (cw_buffer[i - 1].pos == cw_buffer[i].pos)
                && (
                    cw_buffer[i - 1].side == Side::Above && cw_buffer[i].side == Side::Below
                    || cw_buffer[i - 1].side == Side::Below && cw_buffer[i].side == Side::Above
                )
                {
                    remove_idx.push(i - 1);
                }
            }

            for idx in remove_idx.into_iter().rev() {
                cw_buffer.remove(idx);
            }

            if cw_buffer.len() % 2 == 1 {
                dbg!();
                continue;
            }

            for i in (0..cw_buffer.len()).step_by(2) {
                if cw_buffer[i].pos != cw_buffer[i + 1].pos {
                    self.buffer.draw_horizontal_line(cw_buffer[i].pos, cw_buffer[i + 1].pos, y, Vec4::from(1.0));
                }
            }

            /*
            cc_buffer.sort_unstable();

            let mut remove_idx = Vec::new();
            for i in 1..cc_buffer.len() {
                if (
                    cc_buffer[i - 1].left == cc_buffer[i].left
                    || cc_buffer[i - 1].right == cc_buffer[i].right
                )
                && (
                    cc_buffer[i - 1].side == Side::Above && cc_buffer[i].side == Side::Below
                    || cc_buffer[i - 1].side == Side::Below && cc_buffer[i].side == Side::Above
                )
                {
                    remove_idx.push(i);
                }
            }

            for idx in remove_idx.into_iter().rev() {
                cc_buffer.remove(idx);
            }

            if cc_buffer.len() % 2 == 1 {
                dbg!();
                continue;
            }

            for i in (0..cc_buffer.len()).step_by(2) {
                if cc_buffer[i + 1].left <= cc_buffer[i].right + 1 {
                    continue;
                }

                if cc_buffer[i].right != cc_buffer[i + 1].left {
                    self.buffer.draw_horizontal_line(
                        cc_buffer[i].right + 1, 
                        cc_buffer[i + 1].left - 1, 
                        y, 
                        Vec4::new(0.0, 0.0, 0.0, 1.0),
                    );
                }
            }
            */
        }

        // for (_, points) in &self.outlines {
        //     for i in 1..points.len() {
        //         self.buffer.draw_line(points[i - 1].round().cast(), points[i].round().cast(), Vec4::from(1.0));
        //     }

        //     self.buffer.draw_line((*points.last().unwrap()).round().cast(), points[0].round().cast(), Vec4::from(1.0));
        // }

        for (_, points) in &self.outlines {
            for &point in points {
                self.buffer.draw_point(point.round().cast(), Vec4::new(0.0, 1.0, 0.0, 1.0));
            }
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.prev_point = Vec2::default();
        self.outlines.clear();
        self.was_closed = false;
    }
}

impl<'a> OutlineBuilder for GlyphRender<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.prev_point = self.do_transform(Vec2::new(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p1 = self.do_transform(Vec2::new(x, y));
        self.points.push(p1);
        self.prev_point = p1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p1 = self.do_transform(Vec2::new(x1, y1));
        let p2 = self.do_transform(Vec2::new(x, y));

        let t_values = curve::subdivide_quad_curve(self.prev_point.into(), p1.into(), p2.into());

        // Skip 0.0
        for &t in &t_values[1..] {
            let point = curve::get_quad_curve_point(self.prev_point.into(), p1.into(), p2.into(), t);
            self.points.push(point.cast());
        }

        self.prev_point = p2;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p1 = self.do_transform(Vec2::new(x1, y1));
        let p2 = self.do_transform(Vec2::new(x2, y2));
        let p3 = self.do_transform(Vec2::new(x, y));
        
        self.points.push(p1);
        self.points.push(p2);
        self.points.push(p3);

        self.prev_point = p3;
    }

    fn close(&mut self) {
        self.push_outline();
        self.was_closed = true;
    }
}