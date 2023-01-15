use std::ops::Mul;
use std::sync::atomic::{AtomicU16, Ordering};

use image::ImageBuffer;
use rayon::prelude::*;

use crate::basic::colors::Color;
use crate::imaging::PixelPositionIterator;

pub struct Image {

    pixels: Vec<Color>,
    width: u16,
    height: u16,

}

impl Image {

    pub fn new(width: u16, height: u16) -> Self {
        Self::solid(Color::BLACK, width, height)
    }

    pub fn solid(color: Color, width: u16, height: u16) -> Self {
        let w = width as usize;
        let h = height as usize;
        Self {
            pixels: vec![color; w * h],
            width,
            height,
        }
    }

    pub fn init<F: Fn(u16, u16) -> Color>(width: u16, height: u16, initializer: F) -> Self {
        Self {
            pixels: PixelPositionIterator::new(width, height)
                .map(|(i, j, _)| initializer(i, j))
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
        let mut image = (0 .. stack_size).into_par_iter()
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
        image.update_pixels(&|c, _, _| c.mul(ratio).saturated().corrected());
        image
    }

    pub fn update_pixels(&mut self, mapper: &dyn Fn(&Color, u16, u16) -> Color) {
        for (i, j, l) in self.pixel_position_iterator() {
            let pixel = &mut self.pixels[l];
            *pixel = mapper(&pixel, i, j)
        }
    }

    pub fn blend<F: Fn(Color, Color) -> Color>(&self, image: &Image, blender: F) -> Self {
        assert_eq!(self.width, image.width);
        assert_eq!(self.height, image.height);
        Self {
            pixels: self.pixel_position_iterator()
                .map(|(_, _, p)| blender(self.pixels[p], image.pixels[p]))
                .collect(),
            width: self.width,
            height: self.height
        }
    }

    pub fn save(&self, file: &str) {
        let mut buffer = ImageBuffer::new(self.width as u32, self.height as u32);
        for (i, j, l) in self.pixel_position_iterator() {
            buffer.put_pixel(i as u32, j as u32, self.pixels[l].as_rgb())
        }
        buffer.save(file).unwrap_or(());
    }

    fn pixel_position_iterator(&self) -> PixelPositionIterator {
        PixelPositionIterator::new(self.width, self.height)
    }

}

