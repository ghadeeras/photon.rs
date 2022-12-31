pub mod vectors;
pub mod matrices;
pub mod rays;
pub mod cameras;
pub mod sampling;
pub mod colors;
pub mod images;
pub mod worlds;
pub mod things;
pub mod geometries;
pub mod textures;
pub mod materials;
pub mod transforms;
pub mod brdfs;
pub mod builders;
pub mod noise;

pub const EPSILON: f64 = 2.0 * (f32::EPSILON as f64);

pub fn rough_equality(a: f64, b: f64) -> bool {
    return (a - b).abs() <= EPSILON;
}