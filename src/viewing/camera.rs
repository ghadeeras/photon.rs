use std::ops::Mul;
use std::sync::atomic::{AtomicU16, Ordering};

use rayon::prelude::*;

use crate::images::Image;
use crate::viewing::{CameraPixel, Exposure, Lens, Sensor};
use crate::worlds::World;

pub struct Camera {
    pub lens: Lens,
    pub sensor: Sensor,
    pub exposure: Exposure,
    pub samples_per_pixel: u16,
}

impl Camera {

    pub fn shoot<W: World>(&self, world: &W, stack_count: u16) -> Image {
        let counter = AtomicU16::new(0);
        let mut image = (0 .. stack_count).into_par_iter()
            .map(move |_| {
                let image = self.shoot_linear(world);
                let c = counter.fetch_add(1, Ordering::Relaxed);
                println!("Rendered {} frames out of {}", c + 1, stack_count);
                image
            })
            .reduce(|| Image::new(self.sensor.width, self.sensor.height), |i1, i2| {
                i1.blend(&i2, |c1, c2| c1 + c2)
            });
        let ratio = 1.0 / (stack_count as f64);
        image.update_pixels(&|c, _, _| c.mul(ratio).saturated().corrected());
        image
    }

    fn shoot_linear<W: World>(&self, world: &W) -> Image {
        let width = self.sensor.width;
        let height = self.sensor.height;
        let gain = self.sensor.gain / (self.samples_per_pixel as f64);
        let mut image = Image::new(width, height);
        for (j, row) in image.rows.iter_mut().enumerate() {
            for (i, color) in row.pixels.iter_mut().enumerate() {
                let pixel = self.pixel(i, j);
                *color = pixel.estimate_color(world, gain);
            }
        }
        image
    }

    fn pixel(&self, x: usize, y: usize) -> CameraPixel {
        CameraPixel {
            camera: self,
            pixel: self.sensor.pixel(x, y)
        }
    }

}
