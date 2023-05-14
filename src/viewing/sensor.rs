use crate::viewing::Pixel;

pub struct Sensor {
    pub width: usize,
    pub height: usize,
    pub gain: f64,

    aspect: f64,
    pixel_size: f64,
}

impl Sensor {

    pub fn new(width: usize, height: usize, gain: f64) -> Self {
        Self {
            width,
            height,
            gain,
            aspect: (width as f64) / (height as f64),
            pixel_size: 2.0 / (height as f64)
        }
    }

    pub fn pixel(&self, x: usize, y: usize) -> Pixel {
        let size = self.pixel_size;
        let aspect = self.aspect;
        Pixel {
            x: (x as f64) * size - aspect,
            y: 1.0 - (y as f64) * size,
            size,
        }
    }

}
