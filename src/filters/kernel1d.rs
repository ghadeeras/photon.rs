use crate::basic::colors::Color;
use crate::filters::Kernel;
use crate::filters::kernel1d::Orientation::{Horizontal, Vertical};
use crate::imaging::Image;

pub struct Kernel1D(Orientation, Vec<f64>);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {

    fn flipped(self) -> Self {
        if self == Horizontal { Vertical } else { Horizontal }
    }

}

impl Kernel1D {

    pub fn horizontal<F: Fn(i16) -> f64>(half_size: u8, initializer: F) -> Self {
        Self::new(Horizontal, half_size, initializer)
    }

    pub fn vertical<F: Fn(i16) -> f64>(half_size: u8, initializer: F) -> Self {
        Self::new(Vertical, half_size, initializer)
    }

    fn new<F: Fn(i16) -> f64>(orientation: Orientation, half_size: u8, initializer: F) -> Self {
        let size = (((half_size as usize) << 1) + 1) as i16;
        let hs = half_size as i16;
        let weights = (0..size)
            .map(|i| i - hs)
            .map(initializer)
            .collect::<Vec<f64>>();
        Self(orientation, weights)
    }

    pub fn half_size(&self) -> usize {
        self.size() >> 1
    }

    pub fn size(&self) -> usize {
        let Self(_, ref weights) = self;
        weights.len()
    }

    pub fn weights_sum(&self) -> f64 {
        let Self(_, ref weights) = self;
        weights.iter().sum::<f64>()
    }

    pub fn normalize(&self) -> Self {
        let Self(ref orientation, ref weights) = self;
        let sum = self.weights_sum();
        Self(*orientation, weights.iter().map(|w| w / sum).collect())
    }

    pub fn as_vertical(&self) -> Self {
        let Self(_, ref weights) = self;
        Self(Vertical, weights.clone())
    }

    pub fn as_horizontal(&self) -> Self {
        let Self(_, ref weights) = self;
        Self(Vertical, weights.clone())
    }

    pub fn flipped(&self) -> Self {
        let &Self(orientation, ref weights) = self;
        Self(orientation.flipped(), weights.clone())
    }

}

impl Kernel for Kernel1D {

    fn apply_at(&self, x: usize, y: usize, input: &Image) -> Color {
        let &Self(orientation, ref weights) = self;
        let (hsx, hsy) = match orientation {
            Horizontal => (self.half_size(), 0),
            _ => (0, self.half_size())
        };
        let x1 = x.max(hsx) - hsx;
        let y1 = y.max(hsy) - hsy;
        let x2 = (x + hsx + 1).min(input.width());
        let y2 = (y + hsy + 1).min(input.height());
        let first = (x1 + y1 + hsx + hsy) - (x + y);
        let mut color = Color::BLACK;
        for (i, ref pos) in input.pixel_positions_of_rect(x1, y1, x2, y2).enumerate() {
            let weight = weights[first + i];
            color += weight * input[pos]
        }
        color
    }

}
