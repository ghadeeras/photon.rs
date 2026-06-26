use crate::basic::vectors::Vec3D;
use crate::wgpu::data::{Data, Writable};
use crate::wgpu::gpu::GPU;
use crate::wgpu::{initialized_uniform_buffer, storage_buffer};
use wgpu;
use wgpu::Buffer;

pub mod sphere;
pub mod transform;

pub trait Meshable {

    type Generator: MeshGenerator;

    fn generator(&self, gpu: &GPU) -> Self::Generator;

}

pub trait MeshGenerator {

    type Params: Clone;

    fn mesh(&self, input: &Self::Params) -> Mesh {
        let mesh_size = self.mesh_size(&input);
        let mesh = Mesh::new(self.gpu(), mesh_size);
        self.populate_mesh(&input, &mesh);
        mesh
    }

    fn mesh_size(&self, input: &Self::Params) -> MeshSize;

    fn populate_mesh(&self, input: &Self::Params, mesh: &Mesh) -> MeshView;

    fn gpu(&self) -> &GPU;

}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub offset_size: MeshSize,
    pub indices_buffer: Buffer,
    pub positions_buffer: Buffer,
    pub vertices_buffer: Buffer,
}

#[derive(Debug, Clone)]
pub struct MeshSize {
    pub indices_count: u32,
    pub vertices_count: u32,
}

#[derive(Debug)]
pub struct MeshView {
    offset_size: MeshSize,
    size: MeshSize,
    buffer: Option<Buffer>
}

impl Mesh {

    pub fn new(gpu: &GPU, mesh_size: MeshSize) -> Self {
        Self {
            offset_size: MeshSize { indices_count: 0, vertices_count: 0 },
            indices_buffer: storage_buffer(&gpu, "Indices Buffer", mesh_size.indices_size()),
            positions_buffer: storage_buffer(&gpu, "Positions Buffer", mesh_size.vertices_size()),
            vertices_buffer: storage_buffer(&gpu, "Vertices Buffer", mesh_size.vertices_size()),
        }
    }

}

impl MeshSize {

    pub fn empty() -> Self {
        Self {
            indices_count: 0,
            vertices_count: 0,
        }
    }

    pub fn add(&mut self, other: &MeshSize) -> &mut Self {
        self.indices_count += other.indices_count;
        self.vertices_count += other.vertices_count;
        self
    }

    pub fn sub(&mut self, other: &MeshSize) -> &mut Self {
        self.indices_count -= other.indices_count;
        self.vertices_count -= other.vertices_count;
        self
    }

    pub fn indices_size(&self) -> usize {
        self.indices_count as usize * u32::padded_size()
    }

    pub fn positions_size(&self) -> usize {
        self.vertices_count as usize * Vec3D::padded_size()
    }

    pub fn vertices_size(&self) -> usize {
        self.vertices_count as usize * Vec3D::padded_size() * 2
    }

}

impl MeshView {

    pub fn new(offset_size: &MeshSize, mesh_size: &MeshSize) -> Self {
        Self {
            offset_size: offset_size.clone(),
            size: mesh_size.clone(),
            buffer: None,
        }
    }

    pub fn range(offset_size_1: &MeshSize, offset_size_2: &MeshSize) -> Self {
        let mut view = Self {
            offset_size: offset_size_1.clone(),
            size: offset_size_2.clone(),
            buffer: None,
        };
        view.size.sub(offset_size_1);
        view
    }

    pub fn get_buffer_lazily(&mut self, gpu: &GPU) -> &Buffer {
        let buffer = initialized_uniform_buffer(gpu, "Mesh View", &*self);
        self.buffer = Some(buffer);
        self.buffer.as_ref().unwrap()
    }

    pub fn next_offset_size(&self) -> MeshSize {
        let mut s = self.offset_size.clone();
        s.add(&self.size);
        s
    }

}

pub struct ParameterizedMeshable<G: MeshGenerator, M: Meshable<Generator=G>>(
    pub M,
    pub G::Params
);

pub struct ParameterizedMeshGenerator<G: MeshGenerator> {
    generator: G,
    params: G::Params,
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

    fn mesh_size(&self, _: &Self::Params) -> MeshSize {
        self.generator.mesh_size(&self.params)
    }

    fn populate_mesh(&self, _: &Self::Params, mesh: &Mesh) -> MeshView {
        self.generator.populate_mesh(&self.params, mesh)
    }

    fn gpu(&self) -> &GPU {
        self.generator.gpu()
    }

}

impl<G: MeshGenerator, M: Meshable<Generator=G>> Meshable for Vec<M> {

    type Generator = Vec<G>;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        self.iter().map(|m| m.generator(gpu)).collect()
    }

}

impl<G: MeshGenerator> MeshGenerator for Vec<G> {
    type Params = G::Params;

    fn mesh_size(&self, input: &Self::Params) -> MeshSize {
        let mut size = MeshSize::empty();
        for m in self {
            let s = m.mesh_size(input);
            size.add(&s);
        }
        size
    }

    fn populate_mesh(&self, input: &Self::Params, mesh: &Mesh) -> MeshView {
        let mut mut_mesh = mesh.clone();
        for m in self {
            let mesh_view = m.populate_mesh(input, &mut_mesh);
            mut_mesh.offset_size = mesh_view.next_offset_size();
        }
        MeshView::range(&mesh.offset_size, &mut_mesh.offset_size)
    }

    fn gpu(&self) -> &GPU {
        self[0].gpu()
    }

}

impl Data for MeshView {

    fn write<'a>(&self, writable: Writable<'a>) -> Writable<'a> {
        writable
            .write(&self.offset_size.indices_count)
            .write(&self.offset_size.vertices_count)
            .write(&self.size.indices_count)
            .write(&self.size.vertices_count)
    }

    fn padded_size() -> usize {
        4 * u32::padded_size()
    }

}