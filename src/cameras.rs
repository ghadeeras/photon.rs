use rand::{Rng, thread_rng};
use rand::prelude::Distribution;

use crate::{Color, Ray, Vec3D, World};
use crate::images::Image;
use crate::sampling::{UniformSolidUnitCircle, UniformSolidUnitSquare};

pub struct Camera {
    pub lens: Lens,
    pub sensor: Sensor,
    pub exposure: Exposure,
    pub samples_per_pixel: u16,

    gain: f64
}

impl Camera {

    pub fn new(lens: Lens, sensor: Sensor, exposure: Exposure, samples_per_pixel: u16) -> Self {
        let gain = sensor.gain / (samples_per_pixel as f64);
        Self { lens, sensor, exposure, samples_per_pixel, gain }
    }

    pub fn shoot<W: World>(&self, world: &W) -> Image {
        let width = self.sensor.width;
        let height = self.sensor.height;
        let mut image = Image::new(width, height);
        for (j, row) in image.rows.iter_mut().enumerate() {
            for (i, color) in row.pixels.iter_mut().enumerate() {
                let pixel = self.pixel(i, j);
                *color = pixel.estimate_color(world);
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

struct CameraPixel<'a> {
    camera: &'a Camera,
    pixel: Pixel
}

impl<'a> CameraPixel<'a> {

    fn estimate_color<W: World>(&self, world: &W) -> Color {
        let mut color = Color::black();
        for _ in 0u16 .. self.camera.samples_per_pixel {
            let ray = thread_rng().sample(self);
            color += world.trace(&ray);
        }
        color *= self.camera.gain;
        color.saturated().corrected()
    }

}

impl<'a> Distribution<Ray> for CameraPixel<'a> {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ray {
        let lens_sample = rng.sample(&self.camera.lens);
        let pixel_sample = rng.sample(&self.pixel);
        let time = rng.sample(&self.camera.exposure);

        let teleported_pixel_sample = Vec3D::new(pixel_sample.x(), pixel_sample.y(), -self.camera.lens.focal_length);
        let focal_plane_sample = &teleported_pixel_sample * self.camera.lens.projection_ratio;
        let direction = &focal_plane_sample - &lens_sample;
        Ray::new(lens_sample, direction, time)
    }

}

pub struct Lens {
    pub aperture: f64,
    pub focal_length: f64,
    pub focal_plane_distance: f64,

    projection_ratio: f64
}

impl Lens {

    pub fn ideal(focal_length: f64) -> Self {
        Self::new(0.0, focal_length, focal_length)
    }

    pub fn new(aperture: f64, focal_length: f64, focal_plane_distance: f64) -> Self {
        Self {
            aperture,
            focal_length,
            focal_plane_distance,
            projection_ratio: focal_plane_distance / focal_length
        }
    }

}

impl Distribution<Vec3D> for Lens {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        if self.aperture != 0.0 {
            self.aperture * rng.sample(UniformSolidUnitCircle)
        } else {
            Vec3D::zero()
        }
    }

}

pub struct Sensor {
    pub width: u16,
    pub height: u16,
    pub gain: f64,

    aspect: f64,
    pixel_size: f64,
}

impl Sensor {

    pub fn new(width: u16, height: u16, gain: f64) -> Self {
        Self {
            width,
            height,
            gain,
            aspect: (width as f64) / (height as f64),
            pixel_size: 2.0 / (height as f64)
        }
    }

    fn pixel(&self, x: usize, y: usize) -> Pixel {
        let size = self.pixel_size;
        let aspect = self.aspect;
        Pixel {
            x: (x as f64) * size - aspect,
            y: 1.0 - (y as f64) * size,
            size,
        }
    }

}

struct Pixel {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Distribution<Vec3D> for Pixel {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        self.size * rng.sample(UniformSolidUnitSquare) + Vec3D::new(self.x, self.y, 0.0)
    }

}

pub struct Exposure(pub f64);

impl Distribution<f64> for Exposure {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let &Exposure(e) = self;
        if e != 0.0 { -e * rng.gen::<f64>() } else { 0.0 }
    }

}
