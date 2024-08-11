use crate::geometries::Hit;
use crate::Holder;
use crate::materials::{Effect, Material};

pub type MaterialHolder<'a> = Holder<'a, dyn Material>;

impl<'a> Material for MaterialHolder<'a> {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let material = match self {
            &MaterialHolder::Borrowing(m) => m,
            MaterialHolder::Owning(ref m) => m.as_ref()
        };
        material.effect_of(hit)
    }

}
