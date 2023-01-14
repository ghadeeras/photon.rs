use std::sync::Arc;

pub use envs::*;
pub use path_traced::*;

use crate::basic::colors::Color;
use crate::basic::rays::Ray;

mod envs;
mod path_traced;

pub trait World: Send + Sync {

    fn trace(&self, ray: &Ray) -> Color;

}

impl<W: World> World for Arc<W> {

    fn trace(&self, ray: &Ray) -> Color {
        self.as_ref().trace(ray)
    }

}
