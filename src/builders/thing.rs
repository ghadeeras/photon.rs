use crate::builders::Building;
use crate::things::Thing;
use crate::worlds::{PathTraced, World};

impl<T: Thing> Building<T> {

    pub fn with_environment_and_depth<W: World>(self, environment: W, depth: u8) -> Building<PathTraced<W, T>> {
        Building(PathTraced {
            subject: self.done(),
            environment,
            depth
        })
    }

}

