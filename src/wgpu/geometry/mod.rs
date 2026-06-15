use crate::wgpu::gpu::GPU;
use wgpu;

pub mod sphere;
pub mod transform;

pub struct ParameterizedGeometry<M: MeshGenerator, G: Geometry<Generator=M>>(
    pub G,
    pub M::Params
);

pub struct ParameterizedMeshGenerator<M: MeshGenerator> {
    generator: M,
    params: M::Params,
}

pub trait Geometry {

    type Generator: MeshGenerator;

    fn generator(&self, gpu: &GPU) -> Self::Generator;

}

pub trait MeshGenerator {

    type Params: Clone;

    fn mesh(&self, input: &Self::Params) -> Mesh;

    fn gpu(&self) -> &GPU;

}

pub struct Mesh {
    pub indices_buffer: wgpu::Buffer,
    pub positions_buffer: wgpu::Buffer,
    pub vertices_buffer: wgpu::Buffer,
}

impl<M: MeshGenerator, G: Geometry<Generator=M>> Geometry for ParameterizedGeometry<M, G> {

    type Generator = ParameterizedMeshGenerator<M>;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        let &Self(ref geometry, ref params) = self;
        ParameterizedMeshGenerator {
            generator: geometry.generator(gpu),
            params: params.clone(),
        }
    }

}

impl<M: MeshGenerator> MeshGenerator for ParameterizedMeshGenerator<M> {

    type Params = ();

    fn mesh(&self, _: &Self::Params) -> Mesh {
        self.generator.mesh(&self.params)
    }

    fn gpu(&self) -> &GPU {
        self.generator.gpu()
    }

}