use crate::filters::Kernel1D;

pub struct Gaussian {
    pub half_size: u8,
    pub skirt_value: f64
}

impl Gaussian {

    pub fn pipeline(&self) -> Vec<Kernel1D> {
        let hs = self.half_size as f64;
        let c = self.skirt_value.ln() / (hs * hs);
        let h = Kernel1D::horizontal(self.half_size, |x| {
            let xx = x as f64;
            let r2 = xx * xx;
            (c * r2).exp()
        }).normalize();
        let v = h.flipped();
        let p = vec![h, v];
        p
    }

}