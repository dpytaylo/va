// abcdefghijklmnopqrstuvwxyz
use std::mem;

use ttf_parser::OutlineBuilder;

use crate::graphics::rasterizate::SimpleRasterizate;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::curve;
use crate::utils::math::geometry::line::LineEquation;
use crate::utils::math::matrix::matrix3x3::Mat3x3;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;

use super::{buffer::buffer2d::{Buffer2d, Buffer2dRead}, rasterizate::Rasterizate};

#[derive(Clone, Copy)]
enum SweepDirection {
    Clockwise,
    Counterclockwise,
}

pub struct GlyphRenderBuilder {
    transform: Mat3x3<f64>,

    points: Vec<Vec2<f32>>,
    prev_point: Vec2<f32>,
    outlines: Vec<(SweepDirection, Vec<Vec2<f32>>)>,
    was_closed: bool,
}

impl GlyphRenderBuilder {
    pub fn new(transform: Mat3x3<f64>) -> Self {
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
        let vector: Vec3<f64> = Vec3::new(point.x, point.y, 1.0).into();
        (self.transform * vector).xy().cast()
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
        unimplemented!(); // TODO

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

                let mut closure = |p1: Vec2<f32>, p2: Vec2<f32>| {
                    let p1: Vec2<i32> = p1.round().cast();
                    let p2: Vec2<i32> = p2.round().cast();

                    let (min, max) = if p1.y <= p2.y {
                        (p1.y, p2.y)
                    }
                    else {
                        (p2.y, p1.y)
                    };
                    
                    if !(min < y && y <= max) {
                        return;
                    }

                    let equation = LineEquation::new(p1, p2);
                    let x: i32 = equation.get_x_by_y(y.into()).round().cast();
                    buffer.push(x);
                };

                for i in 1..points.len() {
                    closure(points[i - 1], points[i]);
                }

                closure(*points.last().unwrap(), points[0]);

                buffer.sort_unstable();

                let buffers = match dir {
                    SweepDirection::Clockwise => &mut cw_buffers,
                    SweepDirection::Counterclockwise => &mut cc_buffers,
                };

                if !buffer.is_empty() {
                    buffers.push(buffer);
                }                
            }

            for cw_buffer in cw_buffers {
                for i in (0..cw_buffer.len()).step_by(2) {
                    buffer.draw_horizontal_line(
                        cw_buffer[i], 
                        cw_buffer[i + 1],
                        y,
                        Vec4::from(1.0)
                    );
                }
            }

            for cc_buffer in cc_buffers {
                for i in (0..cc_buffer.len()).step_by(2) {
                    buffer.draw_horizontal_line(
                        cc_buffer[i],
                        cc_buffer[i + 1], 
                        y, 
                        Vec4::new(0.0, 0.0, 0.0, 1.0)
                    );
                }
            }
        }

        for (_, points) in &self.outlines {
            for i in 1..points.len() {
                buffer.draw_line(points[i - 1].round().cast(), points[i].round().cast(), Vec4::from(1.0));
            }

            buffer.draw_line((*points.last().unwrap()).round().cast(), points[0].round().cast(), Vec4::from(1.0));
        }
    }
}