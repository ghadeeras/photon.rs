use crate::basic::colors::Color;
use crate::filters::Kernel;
use crate::imaging::Image;

pub struct Kernel2D(Vec<Vec<f64>>);

impl Kernel2D {

    pub fn new<F: Fn(i16, i16) -> f64>(half_size: u8, initializer: F) -> Kernel2D {
        let size = ((half_size as usize) << 1) + 1;
        let hs = half_size as i16;
        let mut weights = vec![vec![0.0; size]; size];
        for (j, row) in weights.iter_mut().enumerate() {
            let y = (j as i16) - hs;
            for (i, weight) in row.iter_mut().enumerate() {
                let x = (i as i16) - hs;
                *weight = initializer(x, y);
            }
        }
        Kernel2D(weights)
    }

    pub fn half_size(&self) -> u8 {
        (self.size() >> 1) as u8
    }

    pub fn size(&self) -> u16 {
        let Self(ref weights) = self;
        weights.len() as u16
    }

    pub fn weights_sum(&self) -> f64 {
        let Self(ref weights) = self;
        weights.iter()
            .map(|row| row.iter().sum::<f64>())
            .sum::<f64>()
    }

    pub fn normalize(&self) -> Kernel2D {
        let Self(ref weights) = self;
        let sum = self.weights_sum();
        let half_size = self.half_size() as usize;
        Self::new(self.half_size(), |x, y| {
            let i = (x as usize) + half_size;
            let j = (y as usize) + half_size;
            weights[j][i] / sum
        })
    }

}

impl Kernel for Kernel2D {

    fn apply_at(&self, x: usize, y: usize, input: &Image) -> Color {
        let Self(ref weights) = self;
        let hs = self.half_size() as usize;
        let x0 = x - hs;
        let y0 = y - hs;
        let x1 = x0.max(0);
        let y1 = y0.max(0);
        let x2 = (x + hs + 1).min(input.width());
        let y2 = (y + hs + 1).min(input.height());
        let mut color = Color::BLACK;
        for pos in input.pixel_positions_of_rect(x1, y1, x2, y2) {
            let weight = weights[pos.row - y0][pos.column - x0];
            color += weight * input[&pos]
        }
        color
    }

}
