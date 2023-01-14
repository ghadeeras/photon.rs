pub use circle::*;
pub use square::*;

mod circle;
mod square;

pub trait PDF<T> {

    fn pdf(&self, value: &T) -> f64;

    fn contains(&self, value: &T) -> bool;

    fn strict_pdf(&self, value: &T) -> f64 {
        if self.contains(value) {
            self.pdf(value)
        } else {
            0.0
        }
    }

}
