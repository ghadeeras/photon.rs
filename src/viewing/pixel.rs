use rand::{Rng, thread_rng};
use rand::prelude::Distribution;

use crate::basic::colors::Color;
use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::sampling::UniformSolidUnitSquare;
use crate::viewing::Camera;
use crate::worlds::World;

pub struct CameraPixel<'a> {
    pub camera: &'a Camera,
    pub pixel: Pixel
}

impl<'a> CameraPixel<'a> {

    pub fn estimate_color<W: World>(&self, world: &W, gain: f64) -> Color {
        let mut color = Color::BLACK;
        for _ in 0u16 .. self.camera.samples_per_pixel {
            let ray = thread_rng().sample(self);
            color += world.trace(&ray);
        }
        color * gain
    }

}

impl<'a> Distribution<Ray> for CameraPixel<'a> {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ray {
        let lens_sample = rng.sample(&self.camera.lens);
        let pixel_sample = rng.sample(&self.pixel);
        let time = rng.sample(&self.camera.exposure);

        let teleported_pixel_sample = Vec3D::new(pixel_sample.x(), pixel_sample.y(), -self.camera.lens.focal_length);
        let focal_plane_sample = &teleported_pixel_sample * self.camera.lens.focal_plane_ratio;
        let direction = &focal_plane_sample - &lens_sample;
        Ray::new(lens_sample, direction, time)
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
