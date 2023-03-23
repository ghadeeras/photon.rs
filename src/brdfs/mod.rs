pub use lambertian::*;

use crate::basic::vectors::Vec3D;
use crate::sampling::Space;

mod lambertian;

/// The Bidirectional Reflectance Distribution Function representing the scattering of some incident
/// ray off some hit surface. The ray direction, the surface normal, and the surface material are
/// implicit for the BRDF.
///
/// Therefore, types implementing this trait might need to hold some form of representation of the
/// incident ray direction, the surface normal, and the surface material. This also means that each
/// ray hit could produce a distinct BRDF that could then be used to perform one of the following
/// operations:
///  * Sampling: Asking for an arbitrary scatter direction.
///  * PDF: Asking for the probability density at a given direction.
///
/// Example:
/// ```
/// # use std::f64::consts::PI;
/// # use photon::brdfs::{BRDF, Lambertian};
/// # use photon::{EPSILON, rough_equality};
/// # use photon::basic::vectors::{Vec3D, Dot};
/// # use photon::sampling::{Space, PDF};
///
/// # let normal = Vec3D::new(1.0, 2.0, 3.0).unit();
///
/// // Lambertian BRDF (typical for matte/diffusive materials)
/// let lambertian = Lambertian::new(&normal);
///
/// // Sampling
/// let (direction, pdf) = lambertian.arbitrary_sample_and_pdf();
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
pub trait BRDF: Space<Vec3D> {

    fn narrowness(&self) -> f64;

}
