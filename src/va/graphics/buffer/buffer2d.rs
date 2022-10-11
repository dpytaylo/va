use std::fmt::Debug;
use std::path::Path;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use anyhow::ensure;
use png::ColorType;

use crate::graphics::image::save_image;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::rect::Rect;
use crate::utils::number::{Number, Float};
use crate::utils::math::vector::vector2::Vec2;
use crate::utils::math::vector::vector4::Vec4;

pub trait Buffer2dRead<T = Self> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> Vec2<usize>;
    fn value(&self, position: Vec2<usize>) -> T;
}

pub trait Buffer2dWrite<T = Self>: Buffer2dRead<T> {
    unsafe fn set_value(&mut self, position: Vec2<usize>, value: T);
}

#[derive(Debug, Default)]
pub struct Buffer2d<T>
    where T: Clone,
{
    size: Vec2<usize>,
    buffer: Vec<T>,
}

impl<T> Buffer2d<T> 
    where T: Clone,
{
    pub fn new(size: Vec2<usize>, value: T) -> Self {
        Self {
            size,
            buffer: vec![value; size.x * size.y],
        }
    }

    pub fn from_iter<Iter>(size: Vec2<usize>, iter: Iter) -> Self 
        where Iter: Iterator<Item = T>,
    {
        let count = size.x * size.y;
        let mut buffer = Vec::with_capacity(count);
        iter.for_each(|val| buffer.push(val));

        Self {
            size,
            buffer,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.buffer.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.buffer.iter_mut()
    }

    pub fn buffer(&self) -> &Vec<T> {
        &self.buffer
    }

    pub fn as_slice(&self) -> Buffer2dSlice<T> {
        let size = self.size.cast();
        Buffer2dSlice {
            buffer: &self,
            rect: Rect::new(Vec2::ZERO, size),
        }
    }

    pub fn as_mut_slice(&mut self) -> Buffer2dMutSlice<T> {
        let size = self.size.cast();
        Buffer2dMutSlice {
            buffer: self,
            rect: Rect::new(Vec2::ZERO, size),
        }
    }

    pub fn slice(&self, rect: Rect<usize>) -> Buffer2dSlice<T> {
        Buffer2dSlice {
            buffer: &self,
            rect,
        }
    }

    pub fn mut_slice(&mut self, rect: Rect<usize>) -> Buffer2dMutSlice<T> {
        Buffer2dMutSlice {
            buffer: self,
            rect,
        }
    }

    // pub fn replace_slice(&mut self, destination: Rect<usize>, slice: Buffer2dSlice<T>) {
    //     assert_eq!(destination.p2 - destination.p1, slice.rect.p2 - slice.rect.p1, "destination rect and slice rect not equal");
    //     assert!(destination.p2 < self.size, "destination rect bigger than the Buffer2d size");

    //     let width = destination.p2.x - destination.p1.x;
    //     let height = destination.p2.y - destination.p1.y;

    //     for i in 0..height {
    //         let scr = &slice.buffer2d().buffer()[i * width] as *const T;
    //         let dst = &mut self.buffer[i * self.size.x + destination.p1.x] as *mut T;

    //         unsafe {
    //             ptr::copy(scr, dst, width);
    //         }
    //     }
    // }
}

impl<T> Clone for Buffer2d<T>
    where T: Clone,
{
    fn clone(&self) -> Self {
        Self { 
            size: self.size, 
            buffer: self.buffer.clone(), 
        }
    }
}

impl<T> IntoIterator for Buffer2d<T>
    where T: Clone,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<T> Buffer2dRead<T> for Buffer2d<T>
    where T: Clone,
{
    fn width(&self) -> usize {
        self.size.x
    }

    fn height(&self) -> usize {
        self.size.y
    }

    fn size(&self) -> Vec2<usize> {
        self.size
    }

    fn value(&self, position: Vec2<usize>) -> T {
        self.buffer[self.size.x * position.y + position.x].clone()
    }
}

impl<T> Buffer2dWrite<T> for Buffer2d<T>
    where T: Clone,
{
    unsafe fn set_value(&mut self, position: Vec2<usize>, value: T) {
        self.buffer[self.size.x * position.y + position.x] = value;
    }
}

impl<T> Buffer2d<Vec4<T>> 
    where T: Number + Float,
          <T as std::ops::Mul>::Output: Float + Cast<u8>,
{
    pub fn to_r8g8b8a8(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.buffer.len() * 4);
        for pixel in &self.buffer {
            for i in 0..4 {
                output.push((pixel[i].clamp(Float::new(0.0), Float::new(1.0)) * Float::new(255.0)).round().cast());
            }
        }

        output
    }
}

pub struct Buffer2dSlice<'a, T> 
    where T: Clone,
{
    buffer: &'a Buffer2d<T>,
    rect: Rect<usize>,
}

impl<'a, T> Buffer2dSlice<'a, T>
    where T: Clone,
{
    fn buffer2d(&self) -> &'a Buffer2d<T> {
        &self.buffer
    }
}

impl<'a, T> Buffer2dRead<T> for Buffer2dSlice<'a, T>
    where T: Clone
{
    fn width(&self) -> usize {
        self.rect.p2.x - self.rect.p1.x
    }

    fn height(&self) -> usize {
        self.rect.p2.y - self.rect.p1.y
    }

    fn size(&self) -> Vec2<usize> {
        self.rect.p2 - self.rect.p1
    }

    fn value(&self, position: Vec2<usize>) -> T {
        self.buffer.value(self.rect.p1 + position)
    }
}

pub struct Buffer2dMutSlice<'a, T> 
    where T: Clone,
{
    buffer: &'a mut Buffer2d<T>,
    rect: Rect<usize>,
}

// impl<'a, T> Buffer2dMutSlice<'a, T> 
//     where T: Clone,
// {
//     /// Note: Instead use Buffer2d "as_mut_slice()" or "mut_slise()" methods
//     fn new(buffer: &'a mut Buffer2d<T>, rect: Rect<usize>) -> Self {
//         Self { 
//             buffer, 
//             rect,
//         }
//     }

//     fn new_virtual_buffer_slice(buffer: &'a mut Buffer2d<T>) {

//     }
// }
 
impl<'a, T> Buffer2dRead<T> for Buffer2dMutSlice<'a, T>
    where T: Clone
{
    fn width(&self) -> usize {
        self.rect.p2.x - self.rect.p1.x
    }

    fn height(&self) -> usize {
        self.rect.p2.y - self.rect.p1.y
    }

    fn size(&self) -> Vec2<usize> {
        self.rect.p2 - self.rect.p1
    }

    fn value(&self, position: Vec2<usize>) -> T {
        self.buffer.value(self.rect.p1 + position)
    }
}

impl<'a, T> Buffer2dWrite<T> for Buffer2dMutSlice<'a, T>
    where T: Clone
{
    unsafe fn set_value(&mut self, position: Vec2<usize>, value: T) {
        self.buffer.set_value(self.rect.p1 + position, value);
    }
}

pub fn save_buffer<T, U>(path: T, buffer: &Buffer2d<Vec4<U>>) -> anyhow::Result<()>
        where T: AsRef<Path>,
              U: Number + Float,
              <U as std::ops::Mul>::Output: Float + Cast<u8>,
{
    ensure!(buffer.width() <= u32::MAX as usize && buffer.height() <= u32::MAX as usize, "Invalid buffer size for saving in a file");
    let data = buffer.to_r8g8b8a8();
    
    save_image(
        path, 
        &data, 
        ColorType::Rgba, 
        buffer.width() as u32, 
        buffer.height() as u32,
    )
}