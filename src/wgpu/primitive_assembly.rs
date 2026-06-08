use crate::wgpu::gpu::GPU;
use wgpu;
use wgpu::wgt::BufferDescriptor;
use crate::wgpu::geometry::Mesh;

pub struct PrimitiveAssembly {
    gpu_pipeline: wgpu::ComputePipeline,
}

pub struct Triangles {
    pub triangles_group: wgpu::BindGroup,
    pub triangles_buffer: wgpu::Buffer,
    pub vertices_buffer: wgpu::Buffer,
}

impl PrimitiveAssembly {

    pub fn new(gpu: &GPU) -> Self {
        let shader = gpu.device.create_shader_module(wgpu::include_wgsl!("./primitive_assembly.wgsl"));
        let gpu_pipeline = gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Primitive Assembly Pipeline"),
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Self { gpu_pipeline }
    }

    pub fn triangles(&self, gpu: &GPU, mesh: &Mesh) -> wgpu::Buffer {
        let triangles_count = mesh.indices_buffer.size() / (3 * 4);
        let triangles_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Triangles Buffer"),
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(wgpu::BufferUsages::STORAGE),
            size: (16 * 4) * triangles_count,
        });
        let triangles_layout = self.gpu_pipeline.get_bind_group_layout(0);
        let triangles_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Triangles Group"),
            layout: &triangles_layout,
            entries: &[
                Self::bind_group_entry(0, &mesh.indices_buffer),
                Self::bind_group_entry(1, &mesh.positions_buffer),
                Self::bind_group_entry(2, &triangles_buffer)
            ]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.gpu_pipeline);
        pass.set_bind_group(0, &triangles_group, &[]);
        pass.dispatch_workgroups(triangles_count.div_ceil(64) as u32, 1, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        triangles_buffer
    }

    #[allow(clippy::needless_lifetimes)]
    fn bind_group_entry<'a>(binding: u32, buffer: &'a wgpu::Buffer) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                size: None,
                offset: 0
            })
        }
    }

}