// abcdefghijklmnopqrstuvwxyz
use std::cell::{RefCell, Ref};
use std::fs;
use std::io::{self, Cursor};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{bail, Context, ensure};
use log::info;
use png::{BitDepth, ColorType};
use thiserror::Error;
use ttf_parser::{Face, OutlineBuilder};

use vulkano::format::Format;
use vulkano::image::view::{ImageView, ImageViewCreateInfo};
use vulkano::image::{ImageDimensions, ImmutableImage, MipmapsCount};
use vulkano::shader::{ShaderCreationError, ShaderModule};

use crate::graphics::buffer::buffer2d::{Buffer2d, Buffer2dRead, save_buffer};
use crate::graphics::glyph_render::{GlyphRender, GlyphRenderBuilder};
use crate::graphics::image::save_image;
use crate::graphics::rasterizate::{Rasterizate, SimpleRasterizate};
use crate::utils::cast::Cast;
use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::matrix::matrix3x3::Mat3x3;
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector3::Vec3;
use crate::utils::math::vector::vector4::Vec4;
use crate::utils::number::{Number, Float};

use super::graphics::mesh::Mesh;
use super::graphics::Graphics;

pub struct Manager {
    graphics: Rc<Graphics>,
    parent_directory: RefCell<String>,
}

#[derive(Debug, Error)]
pub enum ShaderLoadError {
    #[error("io error")]
    IoError(#[from] io::Error),

    #[error("failed to create shader module")]
    FailedToCreateShaderModule(#[from] ShaderCreationError),
}

pub enum FontSize {
    Px(u32),

    /// 1% of viewport
    Vs(u32),
}

impl Manager {
    pub fn new(graphics: Rc<Graphics>) -> Rc<Self> {
        Rc::new(Self {
            graphics,
            parent_directory: RefCell::default(),
        })
    }

    pub fn set_parent_directory(&self, path: &str) {
        *self.parent_directory.borrow_mut() = String::from(path);
    }

    pub fn parent_directory(&self) -> Ref<String> {
        self.parent_directory.borrow()
    }

    pub fn load_text_absolute<T>(&self, filename: T) -> Result<String, io::Error>
        where T: AsRef<Path>,
    {
        fs::read_to_string(filename)
    }

    pub fn load_text_relative<T>(&self, filename: T) -> Result<String, io::Error> 
        where T: AsRef<Path>,
    {
        fs::read_to_string(
            (*self.parent_directory()).clone() 
            + filename.as_ref().to_str().unwrap_or("")
        )
    }

    pub fn load_binary_absolute<T>(&self, filename: T) -> Result<Vec<u8>, io::Error>
        where T: AsRef<Path>,
    {
        fs::read(filename)
    }

    pub fn load_binary_relative<T>(&self, filename: T) -> Result<Vec<u8>, io::Error> 
        where T: AsRef<Path>,
    {
        fs::read(
            (*self.parent_directory()).clone() 
            + filename.as_ref().to_str().unwrap_or("")
        )
    }

    /// # Panics
    ///
    /// Panics if not setup the device
    pub fn create_mesh<T>(&self, vertices: Vec<T>) -> Rc<Mesh<T>>
    where
        T: Copy,
    {
        Mesh::new(vertices)
    }

    /// # Panics
    ///
    /// Panics if not setup the device
    pub fn load_mesh<T>(&self, filename: &str) -> Result<Arc<Mesh<T>>, io::Error>
    where
        T: Copy,
    {
        let _ = filename;
        todo!(); // TODO
    }

    pub fn load_shader(&self, filename: &str) -> Result<Arc<ShaderModule>, ShaderLoadError> {
        // TODO handle errors
        let data = self.load_binary_relative(filename)?;

        let shader_module = unsafe { 
            ShaderModule::from_bytes(self.graphics.device().expect("no device"), &data) 
        }?;

        Ok(shader_module)
    }

    fn parse_png(data: &[u8]) -> anyhow::Result<(ImageDimensions, Vec<u8>)> {
        let cursor = Cursor::new(data);

        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let image_bit_depth = info.bit_depth;
        let image_color_type = info.color_type;

        let mut image_data = vec![0; reader.output_buffer_size()];
        let output_info = reader.next_frame(&mut image_data)?;
        image_data.resize(output_info.buffer_size(), 0);

        Ok((dimensions, match (image_bit_depth, image_color_type) {
            (BitDepth::Eight, ColorType::Rgb) => {
                info!("Using R8G8B8 format texture");

                let pixel_count = image_data.len() / 3;
                let mut new_image = Vec::with_capacity(pixel_count * 4);
                for i in 0..pixel_count {
                    new_image.push(image_data[3 * i]);
                    new_image.push(image_data[3 * i + 1]);
                    new_image.push(image_data[3 * i + 2]);
                    new_image.push(u8::MAX);
                }

                new_image
            }
            (BitDepth::Eight, ColorType::Rgba) => image_data,
            (bit_depth, color_type) => bail!(
                "unsupporting image type (BitDepth::{:?}, ColorType::{:?})",
                bit_depth,
                color_type
            ),
        }))
    }

    /// Loads PNG transparent images(use only R8G8B8A8 format!) and creates the image view
    /// # Panics
    ///
    /// Function panics if not setup queue
    pub fn load_image(&self, filename: &str) -> anyhow::Result<Arc<ImageView<ImmutableImage>>> {
        todo!();
        // let (dimensions, image_data) = Self::parse_png(
        //     &self.load_binary_relative(filename).context("failed to load image binary")?
        // )?;

        // const FORMAT: Format = Format::R8G8B8A8_SRGB;
        // let (image, future) = match ImmutableImage::from_iter(
        //     image_data.into_iter(),
        //     dimensions,
        //     MipmapsCount::One,
        //     FORMAT, // TODO choose srgb or default
        //     self.graphics.queue().expect("no available queue"),
        // ) {
        //     Ok(val) => val,
        //     Err(err) => bail!(format!("{} ({:?})", err, FORMAT)),
        // };
        // dbg!();

        // self.graphics.new_future(Box::new(future));

        // Ok(ImageView::new(image)?)
    }

    pub fn load_image_from_memory(&self, buffer: &Buffer2d<Vec4<f32>>) -> anyhow::Result<Arc<ImageView<ImmutableImage>>> {
        const FORMAT: Format = Format::R8G8B8A8_SRGB;
        let (image, future) = match ImmutableImage::from_iter(
            buffer.to_r8g8b8a8(),
            ImageDimensions::Dim2d {
                width: buffer.width() as u32, 
                height: buffer.height() as u32, 
                array_layers: 1
            },
            MipmapsCount::One,
            FORMAT, // TODO choose srgb or default
            self.graphics.queue().expect("no available queue"),
        ) {
            Ok(val) => val,
            Err(err) => bail!(format!("{} ({:?})", err, FORMAT)),
        };
        
        self.graphics.new_future(Box::new(future));

        Ok(ImageView::new_default(image)?)
    }

    pub fn load_font(&self, font_name: &str, px_size: u32) -> anyhow::Result<Arc<ImageView<ImmutableImage>>>
    {
        if px_size == 0 {
            bail!("invalid font px size ({px_size} px)");
        }

        let data = self.load_binary_relative("fonts/".to_string() + font_name)?;

        //let index = ttf_parser::fonts_in_collection(&data).context("invalid font")?;
        let face = Face::from_slice(&data, 0).context("invalid font")?;

        let capital_height = face.capital_height().context("no capital height")?;
        if capital_height < 1 {
            bail!("invalid capital height ({capital_height})");
        }

        let px_size: f64 = px_size.into();
        let capital_height: f64 = capital_height.into();
        let k = (px_size / capital_height) as f32;

        let mut buffer2d = Buffer2d::new(
            Vec2::new(800, 200),
            Vec4::from(0.0),
        );

        // TODO
        buffer2d.fill(Vec4::new(0.0, 0.0, 0.0, 1.0));
                
        let mut offset = 0.0;
        for char in ('A'..='Z').chain('a'..='z') {
        //for char in 'O'..='O' {
            let glyph_id = face.glyph_index(char).context("invalid glyph index")?;
            let bounding_box = face.glyph_bounding_box(glyph_id).context("invalid glyph bounding box")?;
    
            let bounding_box: Rect<f32> = Rect::new(
                Vec2::new(bounding_box.x_min, bounding_box.y_min).into(),
                Vec2::new(bounding_box.x_max, bounding_box.y_max).into(),
            );

            //dbg!(bounding_box);
    
            //let transform = Mat3x3::with_scale(Vec2::new(k, k));
            let translate = Mat3x3::with_translate(Vec2::new(-bounding_box.p1.x, -bounding_box.p1.y));
            let scale = Mat3x3::with_scale(Vec2::new(k, -k));
    
            let transform = scale * translate;
            
            let height_rect = Rect::new(
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 0.0)).xy(), 
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 0.0)).xy(),
            );

            //dbg!(bounding_box);
            
            let box_height = height_rect.p1.y - height_rect.p2.y;
            //dbg!(box_height);

            let translate = Mat3x3::with_translate(Vec2::new(offset, box_height));
            let transform = translate * transform;
    
            let bounding_box = Rect::new(
                (transform * Vec3::new(bounding_box.p1.x, bounding_box.p1.y, 0.0)).xy(), 
                (transform * Vec3::new(bounding_box.p2.x, bounding_box.p2.y, 0.0)).xy(),
            );

            //dbg!(bounding_box);
    
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

        let size: Vec2<u32> = buffer2d.size().cast();
        let (width, height) = size.into();
        let data = buffer2d.to_r8g8b8a8();

        self.save_image(
            "C:/Users/dpyta/Desktop/output.png", 
            &data, 
            ColorType::Rgba,
            width, 
            height,
        )?;

        const FORMAT: Format = Format::R8G8B8A8_SRGB;
        let (image, future) = match ImmutableImage::from_iter(
            data,
            ImageDimensions::Dim2d {width, height, array_layers: 1},
            MipmapsCount::One,
            FORMAT, // TODO choose srgb or default
            self.graphics.queue().expect("no available queue"),
        ) {
            Ok(val) => val,
            Err(err) => bail!(format!("{} ({:?})", err, FORMAT)),
        };
        
        self.graphics.new_future(Box::new(future));

        Ok(ImageView::new_default(image)?)
    }

    pub fn save_image<T>(&self, path: T, data: &[u8], color_type: ColorType, width: u32, height: u32) -> anyhow::Result<()>
        where T: AsRef<Path>,
    {
        save_image(path, data, color_type, width, height)
    }

    pub fn save_buffer<T, U>(&self, path: T, buffer: &Buffer2d<Vec4<U>>) -> anyhow::Result<()>
        where T: AsRef<Path>,
              U: Number + Float,
              <U as std::ops::Mul>::Output: Float + Cast<u8>,
    {
        save_buffer(path, buffer)
    }
}
