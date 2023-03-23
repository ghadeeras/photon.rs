use crate::builders::Building;
use crate::things::Thing;
use crate::worlds::{ImportantDirectionSampler, PathTraced, World};

impl<W: World, T: Thing, S: ImportantDirectionSampler> Building<PathTraced<W, T, S>> {

    pub fn with_environment<E: World>(self, environment: E) -> Building<PathTraced<E, T, S>> {
        let world = self.done();
        Building(PathTraced {
            subject: world.subject,
            environment,
            depth: world.depth,
            directions_sampler: world.directions_sampler,
        })
    }

    pub fn with_depth(self, depth: u8) -> Building<PathTraced<W, T, S>> {
        let world = self.done();
        Building(PathTraced {
            subject: world.subject,
            environment: world.environment,
            depth,
            directions_sampler: world.directions_sampler,
        })
    }

    pub fn with_directions_sampler<DS: ImportantDirectionSampler>(self, directions_sampler: DS) -> Building<PathTraced<W, T, DS>> {
        let world = self.done();
        Building(PathTraced {
            subject: world.subject,
            environment: world.environment,
            depth: world.depth,
            directions_sampler,
        })
    }

}
