use image::{ImageBuffer, Rgb};

use crate::Color;

pub struct Image {

    pub rows: Vec<ImageRow>,
    width: u16,
    height: u16,

}

impl Image {

    pub fn new(width: u16, height: u16) -> Self {
        let mut image = Self {
            rows: vec![],
            width,
            height
        };
        for _ in 0u16 .. height {
            image.rows.push(ImageRow::new(width))
        }
        image
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
        let mut row = Self { pixels: vec![] };
        for _ in 0u16 .. width {
            row.pixels.push(Color::black())
        }
        row
    }

    fn write_to(&self, buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, j: u32) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            buffer.put_pixel(i as u32, j, pixel.as_rgb())
        }
    }

}