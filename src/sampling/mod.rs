pub use circle::*;
pub use sphere::*;
pub use square::*;

mod circle;
mod square;
mod sphere;

pub trait Space<T>: PDF<T> {

    fn arbitrary_sample_and_pdf(&self) -> (T, f64);

    fn arbitrary_sample(&self) -> T {
        let (sample, _) = self.arbitrary_sample_and_pdf();
        sample
    }

}

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
