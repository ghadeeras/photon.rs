use rand::prelude::Distribution;
use rand::Rng;

pub struct Exposure(pub f64);

impl Distribution<f64> for Exposure {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let &Exposure(e) = self;
        if e != 0.0 { -e * rng.gen::<f64>() } else { 0.0 }
    }

}
