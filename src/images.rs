use image::ImageBuffer;
use crate::Color;

pub struct Image {

    rows: Vec<ImageRow>,
    width: u16,
    height: u16,

}

impl Image {

    pub fn new(width: u16, height: u16) -> Image {
        let mut image = Image {
            rows: vec![],
            width,
            height
        };
        for _ in 0u16 .. height {
            image.rows.push(ImageRow::new(width))
        }
        image
    }

    pub fn get(&self, x: u16, y: u16) -> Color {
        self.rows.get(y as usize).map(|r| r.get(x)).unwrap_or(Color::black())
    }

    pub fn set(&mut self, x: u16, y: u16, color: &Color) {
        for row in self.rows.get_mut(y as usize) {
            row.set(x, color)
        }
    }

    pub fn save(&self, file: &str) {
        let mut buffer = ImageBuffer::new(self.width as u32, self.height as u32);
        for i in 0 .. self.width {
            for j in 0 .. self.height {
                buffer.put_pixel(i as u32, j as u32, self.get(i, j).as_rgb())
            }
        }
        buffer.save(file).unwrap_or(());
    }

}

pub struct ImageRow {

    pixels: Vec<Color>

}

impl ImageRow {

    fn new(width: u16) -> ImageRow {
        let mut row = ImageRow { pixels: vec![] };
        for _ in 0u16 .. width {
            row.pixels.push(Color::black())
        }
        row
    }

    fn get(&self, x: u16) -> Color {
        self.pixels.get(x as usize).copied().unwrap_or(Color::black())
    }

    fn set(&mut self, x: u16, color: &Color) {
        for pixel in self.pixels.get_mut(x as usize) {
            pixel.components = color.components
        }
    }

}