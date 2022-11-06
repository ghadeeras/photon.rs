use image::{ImageBuffer, Rgb};

use crate::Color;

pub struct Image {

    pub rows: Vec<ImageRow>,
    width: u16,
    height: u16,

}

impl Image {

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            rows: (0u16 .. height).map(|_| ImageRow::new(width)).collect(),
            width,
            height
        }
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

    fn new(width: u16) -> Self {
        Self {
            pixels: (0u16 .. width).map(|_| Color::black()).collect()
        }
    }

    fn write_to(&self, buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, j: u32) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            buffer.put_pixel(i as u32, j, pixel.as_rgb())
        }
    }

}