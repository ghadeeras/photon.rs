use crate::basic::colors::Color;

use super::Image;

pub struct Filter(Vec<Vec<f64>>);

impl Filter {

    pub fn new<F: Fn(i16, i16) -> f64>(half_size: u8, initializer: F) -> Filter {
        let size = ((half_size as usize) << 1) + 1;
        let mut weights = vec![vec![0.0; size]; size];
        for (j, row) in weights.iter_mut().enumerate() {
            let y = (j as i16) - (half_size as i16);
            for (i, weight) in row.iter_mut().enumerate() {
                let x = (i as i16) - (half_size as i16);
                *weight = initializer(x, y);
            }
        }
        Filter(weights)
    }

    pub fn with_weights(weights: &[&[f64]]) -> Filter {
        let size = weights.iter()
            .map(| row | row.len())
            .min()
            .unwrap_or(0)
            .min(weights.len()) - 1;
        let half_size = (size >> 1) as i16;
        Self::new(half_size as u8, |x, y| {
            weights[(x + half_size) as usize][(y + half_size) as usize]
        })
    }

    pub fn gaussian(half_size: u8, skirt_value: f64) -> Filter {
        let hs = half_size as f64;

        let c = skirt_value.ln() / (hs * hs);
        Self::new(half_size, |x, y| {
            let xx = x as f64;
            let yy = y as f64;
            let r2 = xx * xx + yy * yy;
            (c * r2).exp()
        })
    }

    pub fn half_size(&self) -> u8 {
        (self.size() >> 1) as u8
    }

    pub fn size(&self) -> u16 {
        let &Self(ref weights) = self;
        weights.len() as u16
    }

    pub fn weights_sum(&self) -> f64 {
        let &Self(ref weights) = self;
        weights.iter()
            .map(|row| row.iter().sum::<f64>())
            .sum::<f64>()
    }

    pub fn normalize(&self) -> Filter {
        let &Self(ref weights) = self;
        let sum = self.weights_sum();
        let half_size = self.half_size() as usize;
        Self::new(self.half_size(), |x, y| {
            let i = (x as usize) + half_size;
            let j = (y as usize) + half_size;
            weights[j][i] / sum
        })
    }

    pub fn filter(&self, image: &Image) -> Image {
        image.map(|_, x, y| self.filter_at(x, y, image))
    }

    pub fn filter_at(&self, x: u16, y: u16, image: &Image) -> Color {
        let &Self(ref weights) = self;
        let hs_u16 = self.half_size() as u16;
        let hs_i16 = hs_u16 as i16;
        let x0 = (x as i16) - hs_i16;
        let y0 = (y as i16) - hs_i16;
        let x1 = x0.max(0) as u16;
        let y1 = y0.max(0) as u16;
        let x2 = (x + hs_u16 + 1).min(image.width);
        let y2 = (y + hs_u16 + 1).min(image.height);
        let mut color = Color::BLACK;
        for pos in image.pixel_positions_of_rect(x1, y1, x2, y2) {
            let (x, y, _) = pos;
            let weight = weights[((y as i16) - y0) as usize][((x as i16) - x0) as usize];
            color += weight * image.pixel_at(&pos)
        }
        color
    }

}