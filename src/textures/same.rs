use crate::geometries::{Geometry, Hit};
use crate::textures::{Black, MaterialHolder, Texture};

pub struct Same;

impl Texture for Same {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder {
        other_side_texture.material(hit, geometry, &Black)
    }

}
