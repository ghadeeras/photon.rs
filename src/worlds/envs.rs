use crate::basic::colors::Color;
use crate::basic::rays::Ray;
use crate::worlds::World;

pub struct PitchBlack;

impl World for PitchBlack {

    fn trace(&self, _: &Ray) -> Color {
        Color::BLACK
    }

}

