use std::sync::Arc;

pub use path_traced::*;

use crate::basic::colors::Color;
use crate::basic::rays::Ray;

mod path_traced;

pub trait World: Send + Sync {

    fn trace(&self, ray: &Ray) -> Color;

}

pub type WorldFunction = fn(&Ray) -> Color;

impl<W: World> World for Arc<W> {

    fn trace(&self, ray: &Ray) -> Color {
        self.as_ref().trace(ray)
    }

}

impl World for WorldFunction {

    fn trace(&self, ray: &Ray) -> Color {
        self(ray)
    }

}

impl World for Color {

    fn trace(&self, _: &Ray) -> Color {
        *self
    }

}