use std::ops::{Add, AddAssign, Div, Mul};
use image::Rgb;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color {
    pub components: [f64; 3]
}

impl Color {

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color { components: [x, y, z] }
    }

    pub fn red(&self) -> f64 {
        self.components[0]
    }

    pub fn green(&self) -> f64 {
        self.components[1]
    }

    pub fn blue(&self) -> f64 {
        self.components[2]
    }

    pub fn saturated(&self) -> Color {
        let max = self.red().max(self.green()).max(self.blue());
        if max <= 1.0 { *self } else { self / max }
    }

    pub fn corrected(&self) -> Color {
        Color { components: self.components.map(|c| c.sqrt()) }
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        Rgb(self.components.map(|c| (c * 255.0) as u8))
    }

}

impl AddAssign<&Color> for Color {

    fn add_assign(&mut self, rhs: &Color) {
        self.components[0] += rhs.red();
        self.components[1] += rhs.green();
        self.components[2] += rhs.blue();
    }

}

impl AddAssign for Color {

    fn add_assign(&mut self, rhs: Color) {
        *self += &rhs;
    }

}

impl Add for &Color {

    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }

}

impl Add for Color {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }

}

impl Mul<f64> for &Color {

    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(
            self.red() * rhs,
            self.green() * rhs,
            self.blue() * rhs,
        )
    }

}

impl Mul<f64> for Color {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }

}

impl Mul<&Color> for f64 {

    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }

}

impl Mul<Color> for f64 {

    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        &rhs * self
    }

}

impl Div<f64> for &Color {

    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }

}

impl Div<f64> for Color {

    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }

}
