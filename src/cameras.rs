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
}

impl Camera {

    fn pixel(&self, x: u16, y: u16) -> CameraPixel {
        CameraPixel {
            camera: self,
            pixel: self.sensor.pixel(x, y)
        }
    }

    pub fn shoot<T: World>(&self, world: &T) -> Image {
        let mut image = Image::new(self.sensor.width, self.sensor.height);
        for i in 0u16 .. self.sensor.width {
            for j in 0u16 .. self.sensor.height {
                let pixel = self.pixel(i, j);
                let mut color = Color::black();
                for _ in 0u16 .. self.samples_per_pixel {
                    let ray = thread_rng().sample(&pixel);
                    color += world.trace(&ray);
                }
                color = color.saturated().corrected();
                image.set(i, j, &color);
            }
        }
        image
    }

}

pub struct CameraPixel<'a> {
    camera: &'a Camera,
    pixel: Pixel
}

impl<'a> Distribution<Ray> for CameraPixel<'a> {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ray {
        let lens_sample = rng.sample(&self.camera.lens);
        let pixel_sample = rng.sample(&self.pixel);
        let time = rng.sample(&self.camera.exposure);

        let teleported_pixel_sample = Vec3D::new(pixel_sample.x(), pixel_sample.y(), -self.camera.lens.focal_length);
        let focal_plane_sample = &teleported_pixel_sample * self.camera.lens.projection_ratio();
        let direction = (&focal_plane_sample - &lens_sample).unit();
        Ray { origin: lens_sample, direction, time }
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

    pub fn projection_ratio(&self) -> f64 {
        self.projection_ratio
    }

}

impl Distribution<Vec3D> for Lens {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        if self.aperture == 0.0 {
            Vec3D::zero()
        } else {
            self.aperture * rng.sample(UniformSolidUnitCircle)
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

    pub fn aspect(&self) -> f64 {
        self.aspect
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    fn pixel(&self, x: u16, y: u16) -> Pixel {
        let size = self.pixel_size();
        Pixel {
            x: ((x % self.width) as f64) * size - self.aspect(),
            y: 1.0 - ((y % self.height) as f64) * size,
            size,
        }
    }

}

pub struct Pixel {
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
        if e == 0.0 { 0.0 } else { -e * rng.gen::<f64>() }
    }

}
