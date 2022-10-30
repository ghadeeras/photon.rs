use crate::materials::Material;
use crate::things::MaterialHit;

pub trait Texture {

    fn material(&self, hit: &MaterialHit) -> &dyn Material;

}
