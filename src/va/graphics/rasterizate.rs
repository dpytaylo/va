use std::cmp::{min, max};

use crate::utils::math::geometry::curve::{QuadCurveIntPointsIter, CubeCurveIntPointsIter};
use crate::utils::math::geometry::line::LinePointsIter;
use crate::utils::math::geometry::point::PointGeometry;
use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::number::Number;
use crate::va::utils::cast::Cast;
use crate::va::utils::math::vector::vector2::Vec2;
use crate::va::utils::math::vector::vector4::Vec4;

use super::buffer::buffer2d::{Buffer2d, Buffer2dWrite};

pub trait SimpleRasterizate<T>: Buffer2dWrite<T> 
    where T: Clone,
{
    fn fill(&mut self, value: T) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_value(Vec2::new(x, y), value.clone());
            }
        }
    }

    fn pixel(&self, position: Vec2<i32>) -> Option<T> {
        let size = (self.size() - 1).cast(); 
        if !position.is_inside_box(Rect::new(Vec2::ZERO, size)) {
            return None;
        }

        let position = position.cast();
        Some(self.value(position))
    }

    fn draw_point(&mut self, point: Vec2<i32>, value: T) {
        if !((0 <= point.x && point.x < self.width() as i32)
            && (0 <= point.y && point.y < self.height() as i32))
        {
            return;
        }

        self.set_value(
            point.cast(),
            value,
        );
    }

    unsafe fn draw_point_unchecked(&mut self, point: Vec2<i32>, value: T) {
        self.set_value(
            point.cast(),
            value,
        );
    }

    fn draw_horizontal_line(&mut self, x0: i32, x1: i32, y: i32, value: T) {
        if y < 0 
            || y as usize + 1 > self.height() 
        {
            return;
        }

        let (x0, x1) = if x1 < x0 {
            (x1, x0)
        }
        else {
            (x0, x1)
        };

        for x in x0..=x1 {
            self.draw_point(Vec2::new(x, y), value.clone());
        }
    }
}

impl<T: Clone, U: Buffer2dWrite<T>> SimpleRasterizate<T> for U {}

pub trait Rasterizate: SimpleRasterizate<Vec4<f32>> {
    fn draw_line(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, color: Vec4<f32>) {
        let size: Vec2<i32> = self.size().cast();
        let rect = Rect::new(Vec2::ZERO, size - 1);

        let (point, point2) = match rect.is_crossing_by_line(p0, p1) {
            Some((p0, p1)) => (p0.cast(), p1.cast()),
            None => {
                if !rect.is_line_segment_inside(
                    p0.into(),
                    p1.into(),
                ) {
                    return;
                }

                (p0, p1)
            }
        };

        LinePointsIter::new(point, point2).for_each(|val|
            unsafe {
                self.draw_point_unchecked(val, color);
            }
        );
    }

    fn draw_dbg_line(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, color: Vec4<f32>) {
        let size: Vec2<i32> = self.size().cast();
        let rect = Rect::new(Vec2::ZERO, size - 1);

        let (point, point2) = match rect.is_crossing_by_line(p0, p1) {
            Some((p0, p1)) => (p0.cast(), p1.cast()),
            None => {
                if !rect.is_line_segment_inside(
                    p0.into(),
                    p1.into(),
                ) {
                    return;
                }

                (p0, p1)
            }
        };

        LinePointsIter::new(point, point2).for_each(|val|
            unsafe {
                self.draw_point_unchecked(val, self.pixel(val).unwrap() + color);
            }
        );
    }

    fn draw_wu_line(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, color: Vec3<f32>) {
        let size: Vec2<i32> = self.size().try_into().unwrap_or(Vec2::MAX);
        let rect = Rect::new(Vec2::ZERO, size - 1);

        let (p0, p1) = match rect.is_crossing_by_line(p0, p1) {
            Some((p0, p1)) => (p0.cast(), p1.cast()),
            None => {
                if !rect.is_line_segment_inside(
                    p0.into(),
                    p1.into(),
                ) {
                    return;
                }

                (p0, p1)
            }
        };

        let xy_len = (p0 - p1).abs();
        
        let (i, j) = if xy_len.x > xy_len.y {
            (0, 1)
        }
        else {
            (1, 0)
        };

        let (p0, p1) = if p0[i] > p1[i] {
            (p1, p0)
        }
        else {
            (p0, p1)
        };
    
        let k = if xy_len[i] == 0 {
            0.0
        }
        else if p0[j] > p1[j] {
            -xy_len[j] as f32 / xy_len[i] as f32
        }
        else {
            xy_len[j] as f32 / xy_len[i] as f32
        };

        // Endpoints
        for i_val in [p0[i], p1[i]] {
            let j_val = k*(i_val - p0[i]) as f32 + p0[j] as f32;

            let mut point = Vec2::default();
            point[i] = i_val;
            point[j] = j_val.floor() as i32;

            let mut point2 = Vec2::default();
            point2[i] = i_val;
            point2[j] = j_val.ceil() as i32;

            let d1 = 1.0 - (j_val - j_val.floor());
            let d2 = 1.0 - (j_val.ceil() - j_val);

            if let Some(bg) = self.pixel(point) {
                let color = mix(
                    Vec4::new(color.x, color.y, color.z, d1), 
                    bg,
                );
                let color = Vec4::new(color.x, color.y, color.z, d1);

                unsafe {
                    self.draw_point_unchecked(point, color);
                }
            }
            
            if let Some(bg) = self.pixel(point2) {
                let color = mix(
                    Vec4::new(color.x, color.y, color.z, d2),
                    bg,
                );
                let color = Vec4::new(color.x, color.y, color.z, d2);

                unsafe {
                    self.draw_point_unchecked(point2, color);
                }
            }
        }
    
        for i_val in p0[i] + 1..p1[i] {
            let j_val = k*(i_val - p0[i]) as f32 + p0[j] as f32;

            let mut point = Vec2::default();
            point[i] = i_val;
            point[j] = j_val.floor() as i32;

            let mut point2 = Vec2::default();
            point2[i] = i_val;
            point2[j] = j_val.ceil() as i32;

            let d1 = 1.0 - (j_val - j_val.floor());
            let d2 = 1.0 - (j_val.ceil() - j_val);

            let color1 = mix(
                Vec4::new(color.x, color.y, color.z, d1), 
                self.pixel(point).unwrap(),
            );

            let color2 = mix(
                Vec4::new(color.x, color.y, color.z, d2),
                self.pixel(point2).unwrap(),
            );

            unsafe {
                self.draw_point_unchecked(point, color1);
                self.draw_point_unchecked(point2, color2);
            }
        }
    }

    fn draw_rect(&mut self, rect: Rect<i32>, color: Vec4<f32>) {
        let left_top = Vec2::new(
            max(0, min((self.width() - 1) as i32, rect.p1.x)), 
            max(0, min((self.height() - 1) as i32, rect.p1.y)),
        );

        let right_bottom = Vec2::new(
            max(0, min(rect.p2.x, (self.width() - 1) as i32)),
            max(0, min(rect.p2.y, (self.height() - 1) as i32)),
        );

        if left_top > right_bottom {
            return;
        }

        for y in left_top.y..right_bottom.y + 1 {
            for x in left_top.x..right_bottom.x + 1  {
                unsafe {
                    self.draw_point_unchecked(Vec2::new(x, y), color);
                }
            }
        }
    }

    fn draw_quad_curve(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>, color: Vec4<f32>) {
        let size: Vec2<_> = self.size().try_into().unwrap_or(Vec2::MAX);
        let rect = Rect::new(Vec2::ZERO, size - 1);

        if !rect.is_crossing_by_line(p0.into(), p1.into()).is_some()
            && !rect.is_crossing_by_line(p1.into(), p2.into()).is_some()
            && !rect.is_line_segment_inside(p0.into(), p1.into())
            && !rect.is_line_segment_inside(p1.into(), p2.into())
        {
            return;
        }

        QuadCurveIntPointsIter::new(p0, p1, p2).for_each(|val|
            unsafe {
                self.draw_point_unchecked(val.round().cast(), color);
            }
        );
    }

    // fn draw_smooth_quad_curve(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>, color: Vec3<f32>) {
    //     let size: Vec2<i64> = self.size().cast();
    //     let rect = Rect::new(Vec2::ZERO, size - 1); 

    //     if !rect.is_crossing_by_line(p0.into(), p1.into()).is_some()
    //         && !rect.is_crossing_by_line(p1.into(), p2.into()).is_some()
    //         && !rect.is_line_segment_inside(p0.into(), p1.into())
    //         && !rect.is_line_segment_inside(p1.into(), p2.into())
    //     {
    //         return;
    //     }

    //     let bounding_box = Rect::new(
    //         Vec2::new(
    //             [p0.x, p1.x, p2.x].into_iter().min().unwrap(),
    //             [p0.y, p1.y, p2.y].into_iter().min().unwrap(),
    //         ),
    //         Vec2::new(
    //             [p0.x, p1.x, p2.x].into_iter().max().unwrap(),
    //             [p0.y, p1.y, p2.y].into_iter().max().unwrap(),
    //         ),
    //     );

    //     let mut tmp_canvas = Buffer2d::new((bounding_box.p2 - bounding_box.p1 + 1).cast(), Vec4::default());
    //     let offset = -bounding_box.p1;

    //     for point in QuadCurvePointsByAxisIter::new(p0, p1, p2, Axis2d::X) {
    //         let offset: Vec2<f64> = offset.into();
    //         anti_aliasing_curve_point(&mut tmp_canvas, point + offset, color);

    //         //let pos: Vec2<i32> = Vec2::new(point.x, point.y.floor()).cast();
    //         //let k = (1.0 - (point.y - point.y.floor())) as f32;

    //         // unsafe {
    //         //     tmp_canvas.draw_point_unchecked(
    //         //         pos + offset,
    //         //         Vec4::new(color.x, color.y, color.z, k),
    //         //     );
    //         // }

    //         // let pos: Vec2<i32> = Vec2::new(point.x, point.y.ceil()).cast();
    //         // let k = (1.0 - (point.y.ceil() - point.y)) as f32;

    //         // unsafe {
    //         //     tmp_canvas.draw_point_unchecked(
    //         //         pos + offset,
    //         //         Vec4::new(color.x, color.y, color.z, k),
    //         //     );
    //         // }
    //     }

    //     for point in QuadCurvePointsByAxisIter::new(p0, p1, p2, Axis2d::Y) {
    //         let offset: Vec2<f64> = offset.into();
    //         anti_aliasing_curve_point(&mut tmp_canvas, point + offset, color);

    //         //let pos: Vec2<i32> = Vec2::new(point.x.floor(), point.y).cast();
    //         //let k = (1.0 - (point.x - point.x.floor())) as f32;

    //         // if let Some(val) = tmp_canvas.pixel(pos + offset) {
    //         //     if val.w < k {
    //         //         unsafe {
    //         //             tmp_canvas.draw_point_unchecked(
    //         //                 pos + offset,
    //         //                 Vec4::new(color.x, color.y, color.z, k),
    //         //             );
    //         //         }
    //         //     }
    //         // }

    //         // let pos: Vec2<i32> = Vec2::new(point.x.ceil(), point.y).cast();
    //         // let k = (1.0 - (point.x.ceil() - point.x)) as f32;

    //         // if let Some(val) = tmp_canvas.pixel(pos + offset) {
    //         //     if val.w < k {
    //         //         unsafe {
    //         //             tmp_canvas.draw_point_unchecked(
    //         //                 pos + offset,
    //         //                 Vec4::new(color.x, color.y, color.z, k),
    //         //             );
    //         //         }
    //         //     }
    //         // }
    //     }

    //     for y in 0..tmp_canvas.height() {
    //         for x in 0..tmp_canvas.width() {
    //             let pos: Vec2<i32> = Vec2::new(x, y).cast();
    //             if let Some(bg) = self.pixel(bounding_box.p1 + pos) {
    //                 let fg = tmp_canvas.pixel(pos).unwrap();
    //                 let new_color = mix(fg, bg);

    //                 unsafe {
    //                     self.draw_point_unchecked(
    //                         bounding_box.p1 + pos,
    //                         new_color,
    //                     );
    //                 }
    //             }
    //         }
    //     }
    // }

    // TODO
    fn draw_smooth_quad_curve(&mut self, p0: Vec2<i32>, p1: Vec2<i32>, p2: Vec2<i32>, color: Vec3<f32>) {
        let fp0: Vec2<_> = p0.into();
        let fp1: Vec2<_> = p1.into();
        let fp2: Vec2<_> = p2.into();

        let length: f64 = ((fp2 - fp1).lenght() + (fp1 - fp0).lenght()) / 10.0;
        let length = ((1.0 + length.sqr()).sqrt() + 2.0).ceil();

        let formula = |t: f64| {
            (1.0 - t).sqr() * fp0 + 2.0 * t * (1.0 - t) * fp1 + t.sqr() * fp2
        };

        let kt = 1.0 / length;
        let mut prev = p0;

        for i in 1..length as usize {
            let next = formula(kt * i as f64).round().cast();
            self.draw_wu_line(prev, next, color);
            prev = next;
        }

        self.draw_wu_line(prev, p2, color);
    }

    fn draw_cube_curve(
        &mut self, 
        p0: Vec2<i32>, 
        p1: Vec2<i32>, 
        p2: Vec2<i32>,
        p3: Vec2<i32>,
        color: Vec4<f32>,
    ) {
        let size: Vec2<_> = self.size().try_into().unwrap_or(Vec2::MAX);
        let rect = Rect::new(Vec2::ZERO, size - 1);

        if !rect.is_crossing_by_line(p0.into(), p1.into()).is_some()
            && !rect.is_crossing_by_line(p1.into(), p2.into()).is_some()
            && !rect.is_crossing_by_line(p2.into(), p3.into()).is_some()
            && !rect.is_line_segment_inside(p0.into(), p1.into())
            && !rect.is_line_segment_inside(p1.into(), p2.into())
            && !rect.is_line_segment_inside(p2.into(), p3.into())
        {
            return;
        }

        CubeCurveIntPointsIter::new(p0, p1, p2, p3).for_each(|val|
            unsafe {
                self.draw_point_unchecked(val.round().cast(), color);
            }
        );
    }

    fn draw_smooth_cube_curve(
        &mut self, 
        p0: Vec2<i32>, 
        p1: Vec2<i32>, 
        p2: Vec2<i32>,
        p3: Vec2<i32>,
        color: Vec3<f32>,
    )
    {
        let fp0: Vec2<_> = p0.into();
        let fp1: Vec2<_> = p1.into();
        let fp2: Vec2<_> = p2.into();
        let fp3: Vec2<_> = p3.into();

        let length: f64 = ((fp3 - fp2).lenght() + (fp2 - fp1).lenght() + (fp1 - fp0).lenght()) / 10.0;
        let length = ((1.0 + length.sqr()).sqrt() + 2.0).ceil();

        let formula = |t: f64| {
            (1.0 - t).powi(3) * fp0 
                + 3.0 * (1.0 - t).sqr() * t * fp1
                + 3.0 * (1.0 - t) * t.sqr() * fp2
                + t.powi(3) * fp3
        };

        let kt = 1.0 / length;
        let mut prev = p0;

        for i in 1..length as usize {
            let next = formula(kt * i as f64).round().cast();
            self.draw_wu_line(prev, next, color);
            prev = next;
        }

        self.draw_wu_line(prev, p3, color);
    }
}

impl Rasterizate for Buffer2d<Vec4<f32>> {}

// https://stackoverflow.com/questions/726549/algorithm-for-additive-color-mixing-for-rgb-values
fn mix(fg: Vec4<f32>, bg: Vec4<f32>) -> Vec4<f32> {
    let mut r = Vec4::default();
    r.w = 1.0 - (1.0 - fg.w) * (1.0 - bg.w);

    if r.w < 1.0e-6 {
        return r; // Fully transparent -- R,G,B not important
    }

    r.x = fg.x * fg.w / r.w + bg.x * bg.w * (1.0 - fg.w) / r.w;
    r.y = fg.y * fg.w / r.w + bg.y * bg.w * (1.0 - fg.w) / r.w;
    r.z = fg.z * fg.w / r.w + bg.z * bg.w * (1.0 - fg.w) / r.w;

    r
}

fn anti_aliasing_curve_point(buffer: &mut Buffer2d<Vec4<f32>>, point: Vec2<f64>, color: Vec3<f32>) {
    // [ ][ ][ ]
    // [ ][*][ ]
    // [ ][ ][ ]

    dbg!(point);

    let middle = point.floor();
    let points = [
        middle - 1.0,
        Vec2::new(middle.x, middle.y - 1.0),
        Vec2::new(middle.x + 1.0, middle.y - 1.0),

        Vec2::new(middle.x - 1.0, middle.y),
        middle,
        Vec2::new(middle.x + 1.0, middle.y),

        Vec2::new(middle.x - 1.0, middle.y + 1.0),
        Vec2::new(middle.x, middle.y + 1.0),
        middle + 1.0,
    ];
    
    for pos in points {
        if let Some(val) = buffer.pixel(pos.cast()) {
            let k = 1.0 - ((pos) - point).lenght().clamp(0.0, 1.0);
            dbg!(pos);
            dbg!(k);
            if k > val.w.into() {
                unsafe {
                    buffer.draw_point_unchecked(pos.cast(), Vec4::new(color.x, color.y, color.z, k as f32));
                }
            }
        }
    }
}