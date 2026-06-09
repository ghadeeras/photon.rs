use crate::wgpu::geometry::{Geometry, MeshGenerator, Mesh};
use crate::wgpu::gpu::GPU;
use wgpu::wgt::BufferDescriptor;

pub struct Sphere(wgpu::ComputePipeline);
pub struct Stacks(pub u16);

struct MeshInfo {
    indices_count: u32,
    vertices_count: u32,
    workgroups_x: u32,
    workgroups_y: u32,
}

impl Geometry for crate::geometries::Sphere {
    type Generator = Sphere;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        Sphere::new(gpu)
    }

}

impl MeshGenerator for Sphere {

    type Params = Stacks;

    fn mesh(&self, gpu: &GPU, input: &Self::Params) -> Mesh {
        let &Stacks(stacks) = input;
        self.mesh(gpu, stacks)
    }

}

impl Sphere {

    fn new(gpu: &GPU) -> Self {
        let shader = gpu.device.create_shader_module(wgpu::include_wgsl!("../shaders/geometry.wgsl"));
        let gpu_pipeline = gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Sphere Mesh Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some("sphere"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Sphere(gpu_pipeline)
    }

    fn mesh(&self, gpu: &GPU, stacks: u16) -> Mesh {
        let &Sphere(ref gpu_pipeline) = self;
        let mesh_info = self.mesh_info(stacks);
        let indices_buffer = Self::buffer(gpu, "Indices Buffer", (mesh_info.indices_count as u64) * 4);
        let positions_buffer = Self::buffer(gpu, "Positions Buffer", (mesh_info.vertices_count as u64) * 4 * 4);
        let vertices_buffer = Self::buffer(gpu, "Vertices Buffer", (mesh_info.vertices_count as u64) * 2 * 4 * 4);
        let mesh_group_layout = gpu_pipeline.get_bind_group_layout(0);
        let mesh_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Mesh Group"),
            layout: &mesh_group_layout,
            entries: &[
                Self::bind_group_entry(0, &indices_buffer),
                Self::bind_group_entry(1, &positions_buffer),
                Self::bind_group_entry(2, &vertices_buffer)
            ]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(gpu_pipeline);
        pass.set_bind_group(0, &mesh_group, &[]);
        pass.dispatch_workgroups(mesh_info.workgroups_x, mesh_info.workgroups_y, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        Mesh {
            indices_buffer,
            positions_buffer,
            vertices_buffer,
        }
    }

    fn buffer(gpu: &GPU, label: &str, size: u64) -> wgpu::Buffer {
        gpu.device.create_buffer(&BufferDescriptor {
            label: Some(label),
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(wgpu::BufferUsages::STORAGE),
            size,
        })
    }

    #[allow(clippy::needless_lifetimes)]
    fn bind_group_entry<'a>(binding: u32, buffer: &'a wgpu::Buffer) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &&buffer,
                size: None,
                offset: 0
            })
        }
    }

    fn mesh_info(&self, stacks: u16) -> MeshInfo {
        let stacks_u32 = stacks as u32;
        let slices_u32 = stacks_u32 * 2;
        MeshInfo {
            indices_count: 6 * slices_u32 * stacks_u32,
            vertices_count: (slices_u32 + 1) * (stacks_u32 + 1),
            workgroups_x: (slices_u32 + 1).div_ceil(8),
            workgroups_y: (stacks_u32 + 1).div_ceil(8),
        }
    }

}