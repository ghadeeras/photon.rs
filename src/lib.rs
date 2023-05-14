pub mod viewing;
pub mod sampling;
pub mod imaging;
pub mod worlds;
pub mod things;
pub mod geometries;
pub mod textures;
pub mod materials;
pub mod transforms;
pub mod brdfs;
pub mod builders;
pub mod noise;
pub mod basic;
pub mod filters;

pub const EPSILON: f64 = 2.0 * (f32::EPSILON as f64);

pub enum Holder<'a, T: 'a + ?Sized> {
    Borrowing(&'a T),
    Owning(Box<T>)
}

pub fn rough_equality(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
