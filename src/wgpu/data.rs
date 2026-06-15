use crate::basic::vectors::Vec3D;
use crate::transforms::{Affine, Linear, Translation};
use wgpu::BufferViewMut;

pub trait Writable {

    fn write<D: Data>(self, data: &D) -> impl Writable;

    fn index(self) -> usize;

}

impl Writable for &mut BufferViewMut {

    fn write<D: Data>(self, data: &D) -> impl Writable {
        let i = data.write(self, 0);
        (self, i)
    }

    fn index(self) -> usize {
        0
    }

}

impl Writable for (&mut BufferViewMut, usize) {

    fn write<D: Data>(self, data: &D) -> impl Writable {
        let (view, index) = self;
        let i = data.write(view, index);
        (view, i)
    }

    fn index(self) -> usize {
        let (_, index) = self;
        index
    }

}

pub trait Data {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize;

    fn padded_size() -> usize;

}

impl Data for Affine {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize {
        let &Self(Linear(ref m1, ref m2, _), ref t) = self;
        (range, index)
            .write(m1.x())
            .write(m1.y())
            .write(m1.z())
            .write(t)
            .write(m2.x())
            .write(m2.y())
            .write(m2.z())
            .write(&Translation(Vec3D::ZERO))
            .index()
    }

    fn padded_size() -> usize {
        8 * Vec3D::padded_size()
    }

}

impl Data for Translation {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize {
        let &Self(ref t) = self;
        t.write(1.0, range, index)
    }

    fn padded_size() -> usize {
        Vec3D::padded_size()
    }

}

impl Data for Vec3D {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize {
        self.write(0.0, range, index)
    }

    fn padded_size() -> usize {
        4 * f64::padded_size()
    }

}

impl Vec3D {

    fn write(&self, w: f64, range: &mut BufferViewMut, index: usize) -> usize {
        (range, index)
            .write(&self.x())
            .write(&self.y())
            .write(&self.z())
            .write(&w)
            .index()
    }

}

impl Data for f64 {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize {
        (*self as f32).write(range, index)
    }

    fn padded_size() -> usize {
        f32::padded_size()
    }

}

impl Data for f32 {

    fn write(&self, range: &mut BufferViewMut, index: usize) -> usize {
        let i1 = index;
        let i2 = index + f32::padded_size();
        range[i1 .. i2].clone_from_slice(&self.to_le_bytes());
        i2
    }

    fn padded_size() -> usize {
        4
    }

}