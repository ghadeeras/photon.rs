use crate::basic::vectors::Vec3D;

pub mod lambertian;

/// The Bidirectional Reflectance Distribution Function representing the scattering of some incident
/// ray off some hit surface. The ray direction, the surface normal, and the surface material are
/// implicit for the BRDF.
///
/// Therefore, types implementing this trait might need to hold some form of representation of the
/// incident ray direction, the surface normal, and the surface material. This also means that each
/// ray hit could produce a distinct BRDF that could then be used to perform one of the following
/// operations:
///  * Sampling: Asking for an arbitrary scatter direction.
///  * PDF: Asking for the probability distribution at a given direction.
///
/// Example:
/// ```
/// # use std::f64::consts::PI;
/// # use photon::brdfs::{BRDF, lambertian::Lambertian};
/// # use photon::{EPSILON, rough_equality};
/// # use photon::basic::vectors::{Vec3D, Dot};
///
/// # let normal = Vec3D::new(1.0, 2.0, 3.0).unit();
///
/// // Lambertian BRDF (typical for matte/diffusive materials)
/// let lambertian = Lambertian::new(&normal);
///
/// // Sampling
/// let (direction, pdf) = lambertian.sample();
/// let cos = normal.dot(direction);
///
/// assert!(rough_equality(direction.length(), 1.0), "directions should have unit length");
/// assert!(cos >= 0.0, "Lambertian materials should not reflect rays below the surface");
/// assert!(rough_equality(pdf, cos / PI), "Lambertian materials have a PDF proportional to cos(theta)");
///
/// // PDF
/// let calculated_pdf = lambertian.pdf(&direction);
/// let impossible_pdf = lambertian.pdf(&(-normal));
///
/// assert!(rough_equality(calculated_pdf, pdf), "we should get same pdf if we pass back the sample direction");
/// assert!(rough_equality(impossible_pdf, 0.0), "PDF of directions going below the surface is 0");
/// ```
pub trait BRDF {

    /// This method returns an arbitrary reflectance direction vector along with its probability
    /// distribution, since calculating it on the fly could be less costly than calling
    /// [pdf](BRDF::pdf) afterwards.
    fn sample(&self) -> (Vec3D, f64);

    /// This method returns the probability density for the specified direction; a measure of how
    /// likely it is that the incident ray gets scattered in the specified direction. The returned
    /// value should be positive, and the integral of this function over all directions should give
    /// 1.0.
    fn pdf(&self, direction: &Vec3D) -> f64;

    /// This method is Similar to [sample](BRDF::sample), except that it does not return the PDF of
    /// the direction, which could be more efficient in the cases where the PDF is not needed.
    fn sample_direction(&self) -> Vec3D {
        let (direction, _) = self.sample();
        direction
    }

}
