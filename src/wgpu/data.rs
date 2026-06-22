use crate::basic::matrices::Matrix;
use crate::basic::vectors::Vec3D;
use crate::transforms::{Affine, Linear, Translation};
use wgpu::BufferViewMut;


pub struct Writable<'a> {
    view: &'a mut BufferViewMut,
    index: usize,
}

impl<'a> Writable<'a> {

    pub fn new(view: &'a mut BufferViewMut) -> Self {
        Self { view, index: 0 }
    }

    pub fn write<D: Data>(self, data: &D) -> Writable<'a> {
        data.write(self)
    }

    fn write_slice(self, data: &[u8]) -> Writable<'a> {
        let Writable { view, index } = self;
        let i1 = index;
        let i2 = index + data.len();
        view[i1 .. i2].clone_from_slice(data);
        Writable { view, index: i2 }
    }

}

pub trait Data {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a>;

    fn padded_size() -> usize;

}

impl Data for Affine {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        let &Self(Linear(ref m1, ref m2, _), ref t) = self;
        writable
            .write(m1)
            .write(t)
            .write(m2)
            .write(&Translation::ZERO)
    }

    fn padded_size() -> usize {
        2 * (Matrix::padded_size() + Vec3D::padded_size())
    }

}

impl Data for Matrix {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        writable
            .write(self.x())
            .write(self.y())
            .write(self.z())
    }

    fn padded_size() -> usize {
        3 * Vec3D::padded_size()
    }

}

impl Data for Translation {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        let &Self(ref t) = self;
        writable.write(t)
    }

    fn padded_size() -> usize {
        Vec3D::padded_size()
    }

}

impl Data for Vec3D {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        self.write(0.0, writable)
    }

    fn padded_size() -> usize {
        4 * f64::padded_size()
    }

}

impl Vec3D {

    fn write<'a>(&self, w: f64, writable: Writable<'a>) -> Writable<'a> {
        writable
            .write(&self.x())
            .write(&self.y())
            .write(&self.z())
            .write(&w)
    }

}

impl Data for f64 {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        writable.write(&(*self as f32))
    }

    fn padded_size() -> usize {
        f32::padded_size()
    }

}

impl Data for f32 {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        writable.write_slice(&self.to_le_bytes())
    }

    fn padded_size() -> usize {
        4
    }

}