// abcdefghijklmnopqrstuvwxyz
use std::cmp::{Ordering, max, min};
use std::mem;
use std::num::NonZeroUsize;
use std::ops::Index;

use ttf_parser::OutlineBuilder;

use crate::graphics::rasterizate::SimpleRasterizate;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::curve;
use crate::utils::math::geometry::line::LineEquation;
use crate::utils::math::matrix::matrix3x3::Mat3x3;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;

use super::buffer::buffer2d::{Buffer2d, Buffer2dRead};
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
    Other,
}

#[derive(Debug)]
struct PointInfo {
    idx: usize,
    pos: i32,
    side: Side,
}

impl PointInfo {
    fn new(idx: usize, pos: i32, side: Side) -> Self {
        Self { 
            idx,
            pos,
            side,
        }
    }
}

impl PartialEq for PointInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.side == other.side
    }
}

impl Eq for PointInfo {}

impl PartialOrd for PointInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.pos.partial_cmp(&other.pos) {
            Some(val) => {
                match val {
                    Ordering::Equal => {
                        (self.side as usize).partial_cmp(&(other.side as usize))
                    }
                    val => Some(val),
                }
            }
            None => None,
        }
    }
}

impl Ord for PointInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct GlyphRenderBuilder {
    transform: Mat3x3<f32>,

    points: Vec<Vec2<f32>>,
    prev_point: Vec2<f32>,
    outlines: Vec<(SweepDirection, Vec<Vec2<f32>>)>,
    was_closed: bool,
}

impl GlyphRenderBuilder {
    pub fn new(transform: Mat3x3<f32>) -> Self {
        Self {
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

    pub fn build(self) -> GlyphRender {
        GlyphRender { 
            outlines: self.outlines,
        }
    }
}

impl OutlineBuilder for GlyphRenderBuilder {
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

pub struct GlyphRender {
    outlines: Vec<(SweepDirection, Vec<Vec2<f32>>)>,
}

impl GlyphRender {
    pub fn rasterizate(&self, buffer: &mut Buffer2d<Vec4<f32>>) {
        for y in 0..buffer.height() as i32 {
            let mut cw_buffers = Vec::new();
            let mut cc_buffers = Vec::new();

            for (dir, points) in &self.outlines {
                let mut buffer = Vec::new();

                let mut closure = |idx: usize, p1: Vec2<f32>, p2: Vec2<f32>| {
                    let p1: Vec2<i32> = p1.round().cast();
                    let p2: Vec2<i32> = p2.round().cast();

                    let (min, max) = if p1.y <= p2.y {
                        (p1.y, p2.y)
                    }
                    else {
                        (p2.y, p1.y)
                    };
                    
                    if y < min || y > max {
                        return;
                    }

                    if p1.y == p2.y {
                        buffer.push(PointInfo::new(idx, p1.x, Side::Other));
                        buffer.push(PointInfo::new(idx, p2.x, Side::Other));
                        return;
                    }

                    let side = if p1.y < y && p2.y > y 
                        || p1.y > y && p2.y < y 
                    {
                        Side::Other
                    }
                    else if p1.y >= y && p2.y >= y {
                        Side::Below // y coordinate down
                    }
                    else {
                        Side::Above
                    };

                    let equation = LineEquation::new(p1, p2);
                    buffer.push(PointInfo::new(idx, equation.get_x_by_y(y.into()).round().cast(), side));
                };

                let mut idx = 0;
                for i in 1..points.len() {
                    closure(idx, points[i - 1], points[i]);
                    idx += 1;
                }

                closure(idx, *points.last().unwrap(), points[0]);

                let buffers = match dir {
                    SweepDirection::Clockwise => &mut cw_buffers,
                    SweepDirection::Counterclockwise => &mut cc_buffers,
                };

                if !buffer.is_empty() {
                    buffers.push(buffer);
                }                
            }

            for cw_buffer in cw_buffers {
                if y == 11 {
                    dbg!(&cw_buffer);
                }

                let max_idx = cw_buffer.len() - 1;
                let cw_buffer = match Self::prepare_for_rendering(y, cw_buffer, max_idx) {
                    Ok(val) => val,
                    Err(_) => continue,
                };

                if y == 11 {
                    dbg!(&cw_buffer);
                }
    
                for i in (0..cw_buffer.len()).step_by(2) {
                    buffer.draw_horizontal_line(cw_buffer[i].pos, cw_buffer[i + 1].pos, y, Vec4::from(1.0));
                }
            }

            for cc_buffer in cc_buffers {
                if y == 11 {
                    dbg!(&cc_buffer);
                }

                let max_idx = cc_buffer.len() - 1;
                let cc_buffer = match Self::prepare_for_rendering(y, cc_buffer, max_idx) {
                    Ok(val) => val,
                    Err(_) => continue,
                };

                if y == 11 {
                    dbg!(&cc_buffer);
                }

                for i in (0..cc_buffer.len()).step_by(2) {
                    if cc_buffer[i].pos + 1 < cc_buffer[i + 1].pos {
                        buffer.draw_horizontal_line(
                            cc_buffer[i].pos + 1,
                            cc_buffer[i + 1].pos - 1, 
                            y, 
                            Vec4::new(0.0, 0.0, 0.0, 1.0)
                        );
                    }
                }
            }
        }

        // for (_, points) in &self.outlines {
        //     for i in 1..points.len() {
        //         buffer.draw_line(points[i - 1].round().cast(), points[i].round().cast(), Vec4::from(1.0));
        //     }

        //     buffer.draw_line((*points.last().unwrap()).round().cast(), points[0].round().cast(), Vec4::from(1.0));
        // }

        for (_, points) in &self.outlines {
            for &point in points {
                buffer.draw_point(point.round().cast(), Vec4::new(0.0, 1.0, 0.0, 1.0));
            }
        }
    }

    fn prepare_for_rendering(y: i32, mut buffer: Vec<PointInfo>, max_idx: usize) -> Result<Vec<PointInfo>, ()> {
        buffer.sort_unstable();
            
        let mut remove_idx = Vec::new();
        for i in 1..buffer.len() {
            if (buffer[i - 1].pos == buffer[i].pos)
                && (
                    buffer[i - 1].side == Side::Above && buffer[i].side == Side::Below
                    || buffer[i - 1].side == Side::Below && buffer[i].side == Side::Above
                )
            {
                remove_idx.push(i - 1);
            }
        }

        for idx in remove_idx.into_iter().rev() {
            buffer.remove(idx);
        }

        let mut first_idx = None;
        for i in (1..buffer.len()).rev() {
            let diff = max(buffer[i].idx, buffer[i - 1].idx) - min(buffer[i].idx, buffer[i - 1].idx); // because usize type

            if diff == 1 || ((buffer[i - 1].idx == max_idx || buffer[i].idx == max_idx) && diff == max_idx) { 
                if first_idx.is_none() {
                    first_idx = Some(i);
                }
            }
            else if let Some(first_idx) = first_idx.take() {
                if first_idx - i >= 2 {
                    buffer.drain(i + 1..first_idx);
                }
            }
        }

        if buffer.len() % 2 == 1 {
            return Err(());
        }

        Ok(buffer)
    }
}