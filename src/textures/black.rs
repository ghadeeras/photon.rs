use crate::geometries::{Geometry, Hit};
use crate::materials::Absorptive;
use crate::textures::{MaterialHolder, Texture};

pub struct Black;

impl Texture for Black {

    fn material<'a>(&'a self, _: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        return MaterialHolder::Borrowing(&Absorptive)
    }

}
