use crate::basic::matrices::Matrix;
use crate::basic::vectors::Vec3D;
use crate::transforms::{Affine, Linear, Translation};
use instant_app::wgpu::data::{Data, Writable};

impl Data for Affine {

    fn write(&self, writable: Writable) -> Writable {
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

    fn write(&self, writable: Writable) -> Writable {
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

    fn write(&self, writable: Writable) -> Writable {
        let &Self(ref t) = self;
        writable.write(t)
    }

    fn padded_size() -> usize {
        Vec3D::padded_size()
    }

}

impl Data for Vec3D {

    fn write(&self, writable: Writable) -> Writable {
        self.write(0.0, writable)
    }

    fn padded_size() -> usize {
        4 * f64::padded_size()
    }

}

impl Vec3D {

    fn write(&self, w: f64, writable: Writable) -> Writable {
        writable
            .write(&self.x())
            .write(&self.y())
            .write(&self.z())
            .write(&w)
    }

}