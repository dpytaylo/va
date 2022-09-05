// abcdefghijklmnopqrstuvwxyz
use std::cmp::Ordering;
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

use super::buffer::buffer2d::{Buffer2d, Buffer2dRead};
use super::rasterizate::Rasterizate;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum SweepDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, PartialEq)]
enum Side {
    None,
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
            let mut cw_buffer = Vec::new();
            let mut none_cw_points = Vec::new();

            let mut cc_buffer = Vec::new();
            let mut none_cc_points = Vec::new();

            for (dir, points) in &self.outlines {
                let (buffer, none_points) = match dir {
                    SweepDirection::Clockwise => (&mut cw_buffer, &mut none_cw_points),
                    SweepDirection::Counterclockwise => (&mut cc_buffer, &mut none_cc_points),
                };

                let mut closure = |p1: Vec2<f32>, p2: Vec2<f32>| {
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
                        none_points.push(p1.x);
                        none_points.push(p2.x);
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

            // if y == 11 {
            //     dbg!(&cw_buffer);
            //     dbg!(&cc_buffer);
            // }

            let cw_buffer = match Self::prepare_for_rendering(cw_buffer, none_cw_points) {
                Ok(val) => val,
                Err(_) => continue,
            };

            for i in (0..cw_buffer.len()).step_by(2) {
                if cw_buffer[i].pos != cw_buffer[i + 1].pos {
                    buffer.draw_horizontal_line(cw_buffer[i].pos, cw_buffer[i + 1].pos, y, Vec4::from(1.0));
                }
            }

            cc_buffer.sort_unstable();
            
            let mut remove_idx = Vec::new();
            for i in 1..cc_buffer.len() {
                if (
                    (cc_buffer[i - 1].pos == cc_buffer[i].pos)
                    && (
                        cc_buffer[i - 1].side == Side::Above && cc_buffer[i].side == Side::Below
                        || cc_buffer[i - 1].side == Side::Below && cc_buffer[i].side == Side::Above
                    )
                )
                    || cc_buffer[i - 1].side == Side::None && cc_buffer[i].side == Side::None
                {
                    remove_idx.push(i - 1);
                }
            }

            for idx in remove_idx.into_iter().rev() {
                cc_buffer.remove(idx);
            }

            if cc_buffer.len() % 2 == 1 {
                continue;
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

    fn prepare_for_rendering(mut buffer: Vec<SegmentInfo>, mut none_points: Vec<i32>) -> Result<Vec<SegmentInfo>, ()> {
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

        none_points.sort_unstable();
        buffer.extend(none_points.into_iter().map(|val| SegmentInfo::new(val, Side::None)));
        buffer.sort_unstable();

        // Start
        let mut last_i = None;
        for i in 0..buffer.len() {
            if buffer[i].side != Side::None {
                break;
            }

            last_i = Some(i);
        }

        if let Some(idx) = last_i {
            buffer.drain(1..=idx);
        }

        // End
        let mut last_i = None;
        for i in (0..buffer.len()).rev() {
            if buffer[i].side != Side::None {
                break;
            }

            last_i = Some(i);
        }

        if let Some(idx) = last_i {
            buffer.drain(idx..buffer.len() - 1);
        }

        // Middle
        let mut i = 0;
        while i < buffer.len() {
            let mut start_i = None;
            for j in i..buffer.len() {
                if buffer[j].side == Side::None {
                    start_i = Some(i);
                    break;
                }
            }

            if start_i.is_none() {
                break;
            }

            for j in start_i.unwrap() + 1..buffer.len() {
                if buffer[j].side != Side::None {
                    if start_i.unwrap() < i {
                        buffer.drain(start_i.unwrap() + 1..i); // Removing Side::None between first and last
                    }
                    
                    break;
                }
            }

            i += 1;
        }
        
        if buffer.len() % 2 == 1 {
            return Err(());
        }

        Ok(buffer)
    }
}