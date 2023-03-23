use std::f64::consts::PI;

use rand::{Rng, thread_rng};

use crate::basic::matrices::Matrix;
use crate::basic::vectors::{Dot, Vec3D};
use crate::brdfs::BRDF;
use crate::sampling::{PDF, Space, UniformSolidUnitSquare};

/// This type represents Lambertian BRDF. It is typically used to implement matte/diffusive
/// materials.
#[derive(Debug)]
pub struct Lambertian(Matrix);

impl Lambertian {

    /// This constructor of Lambertian BRDF takes a vector representing the surface normal at the
    /// point where the incident ray hit it.
    ///
    /// Scattered rays are distributed in the hemisphere above the surface that is centered around
    /// that ray/surface hit point and oriented in the direction of the specified normal vector. The
    /// PDF of the scattered rays is proportional to `cos(theta)`, where `theta` is the angle
    /// between the scattered ray direction and the normal.
    pub fn new(surface_normal: &Vec3D) -> Lambertian {
        Lambertian(Matrix::with_z_alignment(surface_normal))
    }

    pub fn normal(&self) -> &Vec3D {
        let &Lambertian(ref matrix) = self;
        matrix.z()
    }

}

const ONE_PI: f64 = 1.0 / PI;

impl BRDF for Lambertian {

    fn narrowness(&self) -> f64 {
        0.5
    }

}

impl Space<Vec3D> for Lambertian {

    fn arbitrary_sample_and_pdf(&self) -> (Vec3D, f64) {
        let unit_square_sample = thread_rng().sample(UniformSolidUnitSquare);
        let sin_theta_squared = unit_square_sample.x();
        let sin_theta = sin_theta_squared.sqrt();
        let cos_theta = (1.0 - sin_theta_squared).sqrt();
        let phi = 2.0 * PI * unit_square_sample.y();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let &Lambertian(ref matrix) = self;
        let local_direction = Vec3D::new(
            sin_theta * cos_phi,
            sin_theta * sin_phi,
            cos_theta
        );
        (matrix * &local_direction, cos_theta * ONE_PI)
    }

}

impl PDF<Vec3D> for Lambertian {

    fn pdf(&self, direction: &Vec3D) -> f64 {
        self.normal().dot(direction).max(0.0) * ONE_PI
    }

    fn contains(&self, direction: &Vec3D) -> bool {
        self.normal().dot(direction) > 0.0
    }

    fn strict_pdf(&self, direction: &Vec3D) -> f64 {
        self.pdf(direction)
    }

}

#[cfg(test)]
pub mod tests {
    use proptest::*;

    use crate::basic::vectors::tests::unit_vec3;
    use crate::rough_equality;

    use super::*;

    prop_compose! {
        pub fn lambertian()(normal in unit_vec3()) -> Lambertian {
            Lambertian::new(&normal)
        }
    }

    proptest! {

        #[test]
        fn generates_unit_length_directions(lambertian in lambertian()) {
            let direction = lambertian.arbitrary_sample();

            assert!(rough_equality(direction.length(), 1.0));
        }

        #[test]
        fn generates_directions_above_the_surface(normal in unit_vec3()) {
            let lambertian = Lambertian::new(&normal);

            let direction = lambertian.arbitrary_sample();

            let cos_theta = normal.dot(direction);
            assert!(cos_theta >= 0.0);
        }

        #[test]
        fn generates_directions_with_pdf_proportional_to_cos_theta(normal in unit_vec3()) {
            let lambertian = Lambertian::new(&normal);

            let (direction, pdf) = lambertian.arbitrary_sample_and_pdf();

            let cos_theta = direction.dot(normal);
            assert!(rough_equality(pdf, cos_theta / PI));
        }

        #[test]
        fn calculates_pdf_of_a_given_direction(normal in unit_vec3()) {
            let lambertian = Lambertian::new(&normal);

            let direction = lambertian.arbitrary_sample();
            let pdf = lambertian.pdf(&direction);

            let cos_theta = direction.dot(normal);
            assert!(rough_equality(pdf, cos_theta / PI));
        }

    }

}