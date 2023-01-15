use crate::imaging::Image;
use crate::viewing::{CameraPixel, Exposure, Lens, Sensor};
use crate::worlds::World;

pub struct Camera {
    pub lens: Lens,
    pub sensor: Sensor,
    pub exposure: Exposure,
    pub samples_per_pixel: u16,
}

impl Camera {

    pub fn shoot<W: World>(&self, world: &W, stack_size: u16) -> Image {
        Image::stack(
            stack_size,
            || self.shoot_linear(world),
            |counter| println!("Rendered {} frames out of {}", counter, stack_size)
        )
    }

    fn shoot_linear<W: World>(&self, world: &W) -> Image {
        let width = self.sensor.width;
        let height = self.sensor.height;
        let gain = self.sensor.gain / (self.samples_per_pixel as f64);
        let image = Image::init(width, height, |i, j| {
            let pixel = self.pixel(i as usize, j as usize);
            pixel.estimate_color(world, gain)
        });
        image
    }

    fn pixel(&self, x: usize, y: usize) -> CameraPixel {
        CameraPixel {
            camera: self,
            pixel: self.sensor.pixel(x, y)
        }
    }

}
