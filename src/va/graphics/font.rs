use std::cmp::max;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

use anyhow::{bail, Context};
use ttf_parser::{Face, GlyphId};
use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::{ImmutableImage, ImageDimensions, MipmapsCount};
use vulkano::sync::{GpuFuture, NowFuture};

use crate::graphics::buffer;
use crate::graphics::buffer::buffer2d::Buffer2dRead;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;
use crate::utils::math::matrix::matrix3x3::Mat3x3;

use super::buffer::buffer2d::Buffer2d;
use super::glyph_render::GlyphRenderBuilder;
use super::rasterizate::SimpleRasterizate;

const LATIN_ALPHABET_LENGTH: usize = 26;

// TODO pub
pub struct Font {
    pub name: String,
    pub px_size: u32,
    pub chars_info: HashMap<char, CharInfo>,
    pub buffer2d: Buffer2d<Vec4<f32>>,
    pub image: Arc<ImmutableImage>,
}

#[derive(Clone, Copy)]
pub struct CharInfo {
    bounding_box: Rect<i32>,
}

impl CharInfo {
    fn new(bounding_box: Rect<i32>) -> Self {
        Self {
            bounding_box,
        }
    }
}

impl Font {
    pub fn new(name: String, raw_data: &[u8], px_size: u32, queue: Arc<Queue>) 
        -> anyhow::Result<(Self, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>)> 
    {
        if px_size == 0 {
            bail!("invalid font px size ({px_size} px)");
        }

        let face = Face::from_slice(&raw_data, 0).context("invalid font")?;

        let capital_height = face.capital_height().context("no capital height")?;
        if capital_height < 1 {
            bail!("invalid capital height ({capital_height})");
        }

        let px_size_f64: f64 = px_size.into();
        let capital_height: f64 = capital_height.into();
        let k = px_size_f64 / capital_height;

        let mut chars_info: Vec<(GlyphId, Mat3x3<f64>, CharInfo)> = Vec::with_capacity(LATIN_ALPHABET_LENGTH * 2);

        let mut offset = 0;
        let mut max_height = 0;
        
        for char in ('A'..='Z').chain('a'..='z') {
        //for char in 'A'..='A' {
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
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 1.0)).xy(),
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 1.0)).xy(),
            );

            let box_height = height_rect.p1.y - height_rect.p2.y;

            dbg!(box_height);
            dbg!(transform);

            let translate = Mat3x3::with_translate(Vec2::new(offset.into(), box_height));
            let transform = translate * transform;
    
            dbg!(transform);
            dbg!(bounding_box);

            //let transform = Mat3x3::default();

            let mut bounding_box: Rect<i32> = Rect::new(
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 1.0)).xy(), 
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 1.0)).xy(),
            ).round().cast();

            // Rotate bounding box
            bounding_box.p2.y = bounding_box.p1.y;
            bounding_box.p1.y = 0;

            dbg!(bounding_box);
            offset = bounding_box.p2.x + 1;

            max_height = max(max_height, bounding_box.p2.y + 1);
            chars_info.push((glyph_id, transform, CharInfo::new(bounding_box)));
        }

        let mut buffer2d = Buffer2d::new(
            Vec2::new(offset, max_height).cast(),
            Vec4::from(0.0),
        );

        dbg!(buffer2d.size());

        // TODO
        buffer2d.fill(Vec4::new(0.0, 0.0, 0.0, 1.0));
      
        for (glyph_id, transform, char_info) in chars_info {
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

            buffer2d.draw_rect_border(
                char_info.bounding_box.p1,
                char_info.bounding_box.p2,
                Vec4::new(0.0, 1.0, 0.0, 1.0),
            );
        }

        let size: Vec2<u32> = buffer2d.size().cast();
        let (width, height) = size.into();
        let data = buffer2d.to_r8g8b8a8();

        const FORMAT: Format = Format::R8G8B8A8_SRGB;
        let (image, future) = match ImmutableImage::from_iter(
            data,
            ImageDimensions::Dim2d {width, height, array_layers: 1},
            MipmapsCount::One,
            FORMAT, // TODO choose srgb or default
            queue,
        ) {
            Ok(val) => val,
            Err(err) => bail!(format!("{} ({:?})", err, FORMAT)),
        };

        let font = Self {
            name,
            px_size,
            chars_info: HashMap::new(),
            buffer2d,
            image,
        };

        Ok((font, future))
    }
}