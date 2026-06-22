use crate::wgpu::gpu::GPU;
use wgpu;

pub mod sphere;
pub mod transform;

pub struct ParameterizedMeshable<G: MeshGenerator, M: Meshable<Generator=G>>(
    pub M,
    pub G::Params
);

pub struct ParameterizedMeshGenerator<G: MeshGenerator> {
    generator: G,
    params: G::Params,
}

pub trait Meshable {

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

impl<G: MeshGenerator, M: Meshable<Generator=G>> Meshable for ParameterizedMeshable<G, M> {

    type Generator = ParameterizedMeshGenerator<G>;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        let &Self(ref meshable, ref params) = self;
        ParameterizedMeshGenerator {
            generator: meshable.generator(gpu),
            params: params.clone(),
        }
    }

}

impl<G: MeshGenerator> MeshGenerator for ParameterizedMeshGenerator<G> {

    type Params = ();

    fn mesh(&self, _: &Self::Params) -> Mesh {
        self.generator.mesh(&self.params)
    }

    fn gpu(&self) -> &GPU {
        self.generator.gpu()
    }

}