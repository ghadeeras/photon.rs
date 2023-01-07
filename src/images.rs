use image::{ImageBuffer, Rgb};

use crate::basic::colors::Color;

pub struct Image {

    pub rows: Vec<ImageRow>,
    width: u16,
    height: u16,

}

impl Image {

    pub fn new(width: u16, height: u16) -> Self {
        Self::init(width, height, |_, _| Color::BLACK)
    }

    pub fn init<F: Fn(usize, usize) -> Color>(width: u16, height: u16, initializer: F) -> Self {
        Self {
            rows: (0 .. height as usize).map(|j| ImageRow::init(width, |i| initializer(i, j))).collect(),
            width,
            height
        }
    }

    pub fn update_pixels(&mut self, mapper: &dyn Fn(Color, usize, usize) -> Color) {
        for (j, row) in self.rows.iter_mut().enumerate() {
            for (i, pixel) in row.pixels.iter_mut().enumerate() {
                *pixel = mapper(*pixel, i, j)
            }
        }
    }

    pub fn blend<F: Fn(Color, Color) -> Color>(&self, image: &Image, blender: F) -> Self {
        assert_eq!(self.width, image.width);
        assert_eq!(self.height, image.height);
        Self::init(self.width, self.height, |i, j| {
            blender(self.rows[j].pixels[i], image.rows[j].pixels[i])
        })
    }

    pub fn save(&self, file: &str) {
        let mut buffer = ImageBuffer::new(self.width as u32, self.height as u32);
        for (j, row) in self.rows.iter().enumerate() {
            row.write_to(&mut buffer, j as u32)
        }
        buffer.save(file).unwrap_or(());
    }

}

pub struct ImageRow {

    pub pixels: Vec<Color>

}

impl ImageRow {

    pub fn init<F: Fn(usize) -> Color>(width: u16, initializer: F) -> Self {
        Self {
            pixels: (0 .. width as usize).map(initializer).collect()
        }
    }

    fn write_to(&self, buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, j: u32) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            buffer.put_pixel(i as u32, j, pixel.as_rgb())
        }
    }

}