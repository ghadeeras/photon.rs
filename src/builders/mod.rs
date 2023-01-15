#![allow(dead_code)]

use std::sync::Arc;

pub use geometry::*;
pub use thing::*;

use crate::transforms::{Transformation, Transformed};

mod thing;
mod geometry;

pub struct Building<T>(pub T);

impl<T> Building<T> {

    pub fn done(self) -> T {
        let Building(value) = self;
        value
    }

    pub fn boxed(self) -> Box<T> {
        Box::new(self.done())
    }

    pub fn shared(self) -> Arc<T> {
        Arc::new(self.done())
    }

    pub fn transformed<F: Transformation>(self, transformation: F) -> Building<Transformed<T, F>> {
        Building(Transformed {
            subject: self.done(),
            transformation
        })
    }

}
