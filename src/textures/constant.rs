use crate::geometries::{Geometry, Hit};
use crate::materials::Material;
use crate::textures::{MaterialHolder, Texture};

pub struct Constant<M: Material>(pub M);

impl<M: Material> Texture for Constant<M> {

    fn material<'a>(&'a self, _: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder<'a> {
        let &Constant(ref material) = self;
        MaterialHolder::Borrowing(material)
    }

}
