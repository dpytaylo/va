use std::collections::HashMap;

use anyhow::{bail, Context};
use ttf_parser::Face;

use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;
use crate::utils::math::matrix::matrix3x3::Mat3x3;

use super::glyph_render::GlyphRenderBuilder;

const LATIN_ALPHABET_LENGTH: usize = 26;

struct FontInfo {
    px_size: u32,
    chars_info: HashMap<char, CharInfo>,
}

struct CharInfo {
    bounding_box: Rect<i64>,

}

impl FontInfo {
    fn new(raw_data: &[u8], px_size: u32) -> anyhow::Result<Self> {
        if px_size == 0 {
            bail!("invalid font px size ({px_size} px)");
        }

        let face = Face::from_slice(&raw_data, 0).context("invalid font")?;

        face.x_height()

        let capital_height = face.capital_height().context("no capital height")?;
        if capital_height < 1 {
            bail!("invalid capital height ({capital_height})");
        }

        let px_size: f64 = px_size.into();
        let capital_height: f64 = capital_height.into();
        let k = px_size / capital_height;

        let font_info = Vec::with_capacity(LATIN_ALPHABET_LENGTH * 2);
        let mut offset = 0.0;

        for char in ('A'..='Z').chain('a'..='z') {
            let glyph_id = face.glyph_index(char).context("invalid glyph index")?;
            let bounding_box = face.glyph_bounding_box(glyph_id).context("invalid glyph bounding box")?;
    
            let bounding_box: Rect<f64> = Rect::new(
                Vec2::new(bounding_box.x_min, bounding_box.y_min).into(),
                Vec2::new(bounding_box.x_max, bounding_box.y_max).into(),
            );

            let translate = Mat3x3::with_translate(Vec2::new(-bounding_box.p1.x, -bounding_box.p1.y));
            let scale = Mat3x3::with_scale(Vec2::new(k, -k));
    
            let transform = scale * translate;
            
            let height_rect = Rect::new(
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 0.0)).xy(), 
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 0.0)).xy(),
            );

            let box_height = height_rect.p1.y - height_rect.p2.y;

            let translate = Mat3x3::with_translate(Vec2::new(offset, box_height));
            let transform = translate * transform;
    
            let bounding_box = Rect::new(
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 0.0)).xy(), 
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 0.0)).xy(),
            );

        }

        let mut buffer2d = Buffer2d::new(
            Vec2::new(800, 200),
            Vec4::from(0.0),
        );

        // TODO
        buffer2d.fill(Vec4::new(0.0, 0.0, 0.0, 1.0));
                
        for char in ('A'..='Z').chain('a'..='z') {    
            let mut builder = GlyphRenderBuilder::new(transform);
            loop {
                let _ = face.outline_glyph(
                    glyph_id, 
                    &mut builder
                ).context("no raster glyph image")?;
    
                if builder.was_closed() {
                    break;
                }
            };

            let render = builder.build();
            render.rasterizate(&mut buffer2d);

            offset += bounding_box.p2.x;
        }

        buffer
    }
}