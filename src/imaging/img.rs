use std::ops::{Index, IndexMut, Mul};
use std::sync::atomic::{AtomicU16, Ordering};

use image::ImageBuffer;
use rayon::prelude::*;

use crate::basic::colors::Color;
use crate::imaging::{PixelPosition, PixelPositionIterator};

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {

    pub fn new(width: usize, height: usize) -> Self {
        Self::solid(Color::BLACK, width, height)
    }

    pub fn solid(color: Color, width: usize, height: usize) -> Self {
        Self {
            pixels: vec![color; width * height],
            width,
            height,
        }
    }

    pub fn init<F: Fn(usize, usize) -> Color>(width: usize, height: usize, initializer: F) -> Self {
        Self {
            pixels: PixelPositionIterator::new(width, 0..width, 0..height)
                .map(|p| initializer(p.column, p.row))
                .collect(),
            width,
            height
        }
    }

    pub fn stack<S, P>(stack_size: u16, supplier: S, progress: P) -> Image
    where
        S: Sync + Send + Fn() -> Image,
        P: Sync + Send + Fn(u16)
    {
        let counter = AtomicU16::new(0);
        let supplier_ref = &supplier;
        let image: Image = (0 .. stack_size).into_par_iter()
            .map(|_| {
                let img = supplier_ref();
                progress(counter.fetch_add(1, Ordering::Relaxed) + 1);
                img
            })
            .reduce_with(|i1, i2| {
                i1.blend(&i2, |c1, c2| c1 + c2)
            })
            .unwrap_or_else(supplier_ref);
        let ratio = 1.0 / (stack_size as f64);
        image.map(|c, _, _| c.mul(ratio))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn new_with_same_size(&self) -> Self {
        Self::new(self.width, self.height)
    }

    pub fn copy_to(&self, image: &mut Image, iterator: PixelPositionIterator) {
        for ref p in iterator {
            image[p] = self[p];
        }
    }

    pub fn to_non_linear_space(&self) -> Image {
        self.map(|c, _, _| c.saturated().corrected())
    }

    pub fn map<F: Fn(&Color, usize, usize) -> Color>(&self, mapper: F) -> Image {
        let mut image = self.clone();
        for ref p in self.pixel_position_iterator() {
            let pixel = &mut image[p];
            *pixel = mapper(pixel, p.column, p.row)
        }
        image
    }

    pub fn blend<F: Fn(Color, Color) -> Color>(&self, image: &Image, blender: F) -> Self {
        assert_eq!(self.width, image.width);
        assert_eq!(self.height, image.height);
        Self {
            pixels: self.pixel_position_iterator()
                .map(|ref p| blender(self[p], image[p]))
                .collect(),
            width: self.width,
            height: self.height
        }
    }

    pub fn blend_into<F: Fn(Color, Color) -> Color>(&self, output: &mut Image, image: &Image, blender: F) {
        assert_eq!(self.width, image.width);
        assert_eq!(self.height, image.height);
        self.pixel_position_iterator()
            .for_each(|ref p| output[p] = blender(self[p], image[p]));
    }

    pub fn save(&self, file: &str) {
        let mut buffer = ImageBuffer::new(self.width as u32, self.height as u32);
        for ref p in self.pixel_position_iterator() {
            buffer.put_pixel(p.column as u32, p.row as u32, self[p].as_rgb())
        }
        buffer.save(file).unwrap_or(());
    }

    pub fn pixel_position_iterator(&self) -> PixelPositionIterator {
        self.pixel_positions_of_rect(0, 0, self.width, self.height)
    }

    pub fn pixel_positions_of_rect(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> PixelPositionIterator {
        PixelPositionIterator::new(self.width, x1..x2, y1..y2)
    }

}

impl Index<&PixelPosition> for Image {

    type Output = Color;

    fn index(&self, position: &PixelPosition) -> &Self::Output {
        &self.pixels[position.linear]
    }
}

impl IndexMut<&PixelPosition> for Image {

    fn index_mut(&mut self, position: &PixelPosition) -> &mut Self::Output {
        &mut self.pixels[position.linear]
    }

}
