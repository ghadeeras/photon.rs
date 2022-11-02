use std::rc::Rc;
use crate::materials::Material;
use crate::things::MaterialHit;

pub trait Texture {

    fn material(&self, hit: &MaterialHit) -> &dyn Material;

}

pub struct Constant(pub Rc<dyn Material>);

impl Texture for Constant {

    fn material(&self, _: &MaterialHit) -> &dyn Material {
        let &Constant(ref material) = self;
        material.as_ref()
    }

}

