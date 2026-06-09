use crate::wgpu::gpu::GPU;
use wgpu;

pub mod sphere;

pub trait Geometry: crate::geometries::Geometry {

    type Generator: MeshGenerator;

    fn generator(&self, gpu: &GPU) -> Self::Generator;

}

pub trait MeshGenerator {

    type Params;

    fn mesh(&self, gpu: &GPU, input: &Self::Params) -> Mesh;

}

pub struct Mesh {
    pub indices_buffer: wgpu::Buffer,
    pub positions_buffer: wgpu::Buffer,
    pub vertices_buffer: wgpu::Buffer,
}