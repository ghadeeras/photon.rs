use crate::basic::colors::Color;
use crate::builders::Building;
use crate::things::Thing;
use crate::worlds::{Omnidirectional, PathTraced};

impl<T: Thing> Building<T> {

    pub fn path_traced(self) -> Building<PathTraced<Color, T, Omnidirectional>> {
        Building(PathTraced {
            subject: self.done(),
            environment: Color::BLACK,
            depth: 8,
            directions_sampler: Omnidirectional
        })
    }

}
