use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign};

use image::Rgb;

/// This structure, as the name suggests, represents a color, with its red, green, and blue
/// components.
///
/// Example:
/// ```
/// # use photon::basic::colors::Color;
/// # use photon::rough_equality;
///
/// let bluish = Color::new(0.2, 0.4, 0.8);
/// assert_eq!(bluish.red(), 0.2);
/// assert_eq!(bluish.green(), 0.4);
/// assert_eq!(bluish.blue(), 0.8);
///
/// let darker_red = Color::red_shade(0.2);
/// assert_eq!(darker_red, 0.2 * Color::RED);
///
/// let dark_green = Color::green_shade(0.4);
/// assert_eq!(dark_green, 0.4 * Color::GREEN);
///
/// let blue = Color::blue_shade(0.8);
/// assert_eq!(blue, 0.8 * Color::BLUE);
///
/// let mut mixture = darker_red + dark_green;
/// mixture += blue;
/// assert_eq!(mixture, bluish);
///
/// let reddish = Color::new(0.8, 0.4, 0.2);
/// let actual_prod = bluish * reddish;
/// let expected_prod = Color::grey_shade(0.16);
/// assert!(rough_equality(actual_prod[0], expected_prod[0]));
/// assert!(rough_equality(actual_prod[1], expected_prod[1]));
/// assert!(rough_equality(actual_prod[2], expected_prod[2]));
/// ```
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color {
    components: [f64; 3]
}

impl Color {

    pub const BLACK: Self = Self::grey_shade(0.0);
    pub const WHITE: Self = Self::grey_shade(1.0);
    pub const RED: Self = Self::red_shade(1.0);
    pub const GREEN: Self = Self::green_shade(1.0);
    pub const BLUE: Self = Self::blue_shade(1.0);

    pub const fn grey_shade(shade: f64) -> Self {
        Self::new(shade, shade, shade)
    }
    pub const fn red_shade(shade: f64) -> Self {
        Self::new(shade, 0.0, 0.0)
    }
    pub const fn green_shade(shade: f64) -> Self {
        Self::new(0.0, shade, 0.0)
    }
    pub const fn blue_shade(shade: f64) -> Self {
        Self::new(0.0, 0.0, shade)
    }

    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { components: [red, green, blue] }
    }

    pub fn read_from(slice: &[f64]) -> Self {
        Self::new(slice[0], slice[1], slice[0])
    }

    pub fn write_into(&self, slice: &mut[f64]) {
        slice[0..3].copy_from_slice(&self.components);
    }

    #[inline]
    pub fn red(&self) -> f64 {
        self[0]
    }

    #[inline]
    pub fn green(&self) -> f64 {
        self[1]
    }

    #[inline]
    pub fn blue(&self) -> f64 {
        self[2]
    }

    pub fn luminance(&self) -> f64 {
        0.212639 * self.red() + 0.715169 * self.green() + 0.072192 * self.blue()
    }

    pub fn saturated(&self) -> Self {
        fn sat(c: f64) -> f64 {
            if c < 1.0 { c * (c * (1.0 - c) + 1.0) } else { 1.0 }
        }
        Self::new(
            sat(self.red()),
            sat(self.green()),
            sat(self.blue()),
        )
    }

    pub fn corrected(&self) -> Self {
        Color::new(
            self[0].sqrt(),
            self[1].sqrt(),
            self[2].sqrt(),
        )
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        Rgb(self.components.map(|c| (c * 255.0).round() as u8))
    }

    pub fn plus(&self, rhs: &Self) -> Self {
        Self::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
        )
    }

    pub fn times(&self, rhs: f64) -> Self {
        Self::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        )
    }

    pub fn modulate(&self, rhs: &Self) -> Self {
        Self::new(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
        )
    }
}

impl Index<usize> for Color {

    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }

}

impl IndexMut<usize> for Color {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }

}

impl AddAssign<&Color> for Color {

    fn add_assign(&mut self, rhs: &Color) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
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
        self.plus(rhs)
    }

}

impl Add for Color {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }

}

impl MulAssign<f64> for Color {

    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }

}

impl MulAssign<&Color> for Color {

    fn mul_assign(&mut self, rhs: &Color) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }

}

impl MulAssign<Color> for Color {

    fn mul_assign(&mut self, rhs: Color) {
        *self *= &rhs;
    }

}

impl Mul<&Color> for &Color {

    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        self.modulate(rhs)
    }

}

impl Mul<Color> for Color {

    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        self.modulate(&rhs)
    }

}

impl Mul<f64> for &Color {

    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        )
    }

}

impl Mul<f64> for Color {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.times(rhs)
    }

}

impl Mul<&Color> for f64 {

    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs.times(self)
    }

}

impl Mul<Color> for f64 {

    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs.times(self)
    }

}

impl Div<f64> for &Color {

    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self.times(1.0 / rhs)
    }

}

impl Div<f64> for Color {

    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.times(1.0 / rhs)
    }

}

#[cfg(test)]
pub mod tests {
    use proptest::{*, strategy::*};

    use super::*;

    prop_compose! {
        pub fn range()(v in 0.0..1.0) -> f64 {
            v
        }
    }

    prop_compose! {
        pub fn color()(red in range(), green in range(), blue in range()) -> Color {
            Color::new(red, green, blue)
        }
    }

    prop_compose! {
        pub fn non_black()(c in color().prop_filter("non-black colors", |c| c.luminance() != 0.0)) -> Color {
            c
        }
    }

}