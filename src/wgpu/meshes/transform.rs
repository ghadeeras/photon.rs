use crate::transforms::Affine;
use crate::wgpu::bind_group_entry;
use crate::wgpu::data::{Data, Writable};
use crate::wgpu::gpu::GPU;
use crate::wgpu::meshes::{Mesh, MeshGenerator, Meshable};
use wgpu::wgt::BufferDescriptor;
use wgpu::Buffer;

pub struct TransformedMeshable<G: MeshGenerator, M: Meshable<Generator=G>> {
    pub meshable: M,
    pub transformation: Affine
}

pub struct TransformedMeshGenerator<G: MeshGenerator> {
    generator: G,
    gpu_pipeline: wgpu::ComputePipeline,
    transformation: Buffer
}

impl<G: MeshGenerator, M: Meshable<Generator=G>> Meshable for TransformedMeshable<G, M> {
    type Generator = TransformedMeshGenerator<G>;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        let shader = gpu.device.create_shader_module(wgpu::include_wgsl!("../shaders/transform.wgsl"));
        let gpu_pipeline = gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Transform Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        TransformedMeshGenerator {
            generator: self.meshable.generator(gpu),
            gpu_pipeline,
            transformation: self.transformation.to_buffer(gpu),
        }
    }

}

impl<G: MeshGenerator> MeshGenerator for TransformedMeshGenerator<G> {

    type Params = G::Params;

    fn mesh(&self, input: &Self::Params) -> Mesh {
        let gpu = self.gpu();
        let gpu_pipeline = &self.gpu_pipeline;
        let mesh = self.generator.mesh(input);
        let mesh_group_layout = gpu_pipeline.get_bind_group_layout(0);
        let mesh_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Group"),
            layout: &mesh_group_layout,
            entries: &[
                bind_group_entry(0, &self.transformation),
                bind_group_entry(1, &mesh.positions_buffer),
                bind_group_entry(2, &mesh.vertices_buffer)
            ]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(gpu_pipeline);
        pass.set_bind_group(0, &mesh_group, &[]);
        pass.dispatch_workgroups(mesh.positions_buffer.size().div_ceil(4 * 4 * 64) as u32, 1, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        mesh
    }

    fn gpu(&self) -> &GPU {
        self.generator.gpu()
    }

}

impl Affine {

    fn to_buffer(&self, gpu: &GPU) -> Buffer {
        let buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Transformation"),
            mapped_at_creation: true,
            usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(wgpu::BufferUsages::UNIFORM),
            size: Affine::padded_size() as u64,
        });
        let mut range = buffer.get_mapped_range_mut(..);
        Writable::new(&mut range).write(self);
        drop(range);
        buffer.unmap();
        buffer
    }

}
