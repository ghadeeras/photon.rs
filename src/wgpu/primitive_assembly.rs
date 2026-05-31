use crate::wgpu::gpu::GPU;
use wgpu;
use wgpu::wgt::BufferDescriptor;

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
            label: None,
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Self { gpu_pipeline }
    }

    pub fn new_triangles(gpu: &GPU) -> Triangles {
        let primitive_assembly = PrimitiveAssembly::new(&gpu);
        primitive_assembly.triangles(&gpu)
    }

    pub fn triangles(&self, gpu: &GPU) -> Triangles {
        let triangles_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(wgpu::BufferUsages::STORAGE),
            size: 64 * 16 * 4,
        });
        let vertices_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(wgpu::BufferUsages::STORAGE),
            size: 64 * 2 * 4 * 4,
        });
        let triangles_layout = self.gpu_pipeline.get_bind_group_layout(0);
        let triangles_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &triangles_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &triangles_buffer,
                    size: None,
                    offset: 0
                })
            }, wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &vertices_buffer,
                    size: None,
                    offset: 0
                })
            }]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.gpu_pipeline);
        pass.set_bind_group(0, &triangles_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        Triangles { triangles_group, triangles_buffer, vertices_buffer }
    }

}