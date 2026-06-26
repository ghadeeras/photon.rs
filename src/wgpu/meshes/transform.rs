use crate::transforms::Affine;
use crate::wgpu::gpu::GPU;
use crate::wgpu::meshes::{Mesh, MeshGenerator, MeshSize, MeshView, Meshable};
use crate::wgpu::{bind_group_buffer_entry, initialized_uniform_buffer};
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
            transformation: initialized_uniform_buffer(gpu, "Transformation", &self.transformation),
        }
    }

}

impl<G: MeshGenerator> MeshGenerator for TransformedMeshGenerator<G> {

    type Params = G::Params;

    fn mesh_size(&self, input: &Self::Params) -> MeshSize {
        self.generator.mesh_size(input)
    }

    fn populate_mesh(&self, input: &Self::Params, mesh: &Mesh) -> MeshView {
        let gpu = self.gpu();
        let gpu_pipeline = &self.gpu_pipeline;
        let mut mesh_view = self.generator.populate_mesh(input, mesh);
        let mesh_size = self.mesh_size(input);
        let mesh_group_layout = gpu_pipeline.get_bind_group_layout(0);
        let mesh_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Group"),
            layout: &mesh_group_layout,
            entries: &[
                bind_group_buffer_entry(0, mesh_view.get_buffer_lazily(self.gpu())),
                bind_group_buffer_entry(1, &self.transformation),
                bind_group_buffer_entry(2, &mesh.positions_buffer),
                bind_group_buffer_entry(3, &mesh.vertices_buffer)
            ]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(gpu_pipeline);
        pass.set_bind_group(0, &mesh_group, &[]);
        pass.dispatch_workgroups(mesh_size.vertices_count.div_ceil(64), 1, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        mesh_view
    }

    fn gpu(&self) -> &GPU {
        self.generator.gpu()
    }

}
