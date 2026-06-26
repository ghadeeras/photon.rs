use crate::wgpu::bind_group_buffer_entry;
use crate::wgpu::gpu::GPU;
use crate::wgpu::meshes::{Mesh, MeshGenerator, MeshSize, MeshView, Meshable};

pub struct Sphere {
    gpu: GPU,
    gpu_pipeline: wgpu::ComputePipeline,
}

#[derive(Clone)]
pub struct SphereParams {
    pub latitudes: u16,
    pub longitudes: u16,
}

#[derive(Debug)]
struct MeshInfo {
    size: MeshSize,
    workgroups_x: u32,
    workgroups_y: u32,
}

impl Meshable for crate::geometries::Sphere {

    type Generator = Sphere;

    fn generator(&self, gpu: &GPU) -> Self::Generator {
        Sphere::new(gpu)
    }

}

impl MeshGenerator for Sphere {

    type Params = SphereParams;

    fn mesh_size(&self, input: &Self::Params) -> MeshSize {
        let mesh_info = self.mesh_info(input.longitudes, input.latitudes);
        mesh_info.size
    }

    fn populate_mesh(&self, input: &Self::Params, mesh: &Mesh) -> MeshView {
        self.populate_mesh(input.longitudes, input.latitudes, mesh)
    }

    fn gpu(&self) -> &GPU {
        &self.gpu
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
        Sphere {
            gpu: gpu.clone(),
            gpu_pipeline
        }
    }

    fn populate_mesh(&self, longitudes: u16, latitudes: u16, mesh: &Mesh) -> MeshView {
        let &Sphere { ref gpu_pipeline, ref gpu } = self;
        let mesh_info = self.mesh_info(longitudes, latitudes);
        log::info!("mesh info: {:?}", mesh_info);
        let mut mesh_view = MeshView::new(&mesh.offset_size, &mesh_info.size);
        let mesh_group_layout = gpu_pipeline.get_bind_group_layout(0);
        let mesh_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Mesh Group"),
            layout: &mesh_group_layout,
            entries: &[
                bind_group_buffer_entry(0, mesh_view.get_buffer_lazily(self.gpu())),
                bind_group_buffer_entry(1, &mesh.indices_buffer),
                bind_group_buffer_entry(2, &mesh.positions_buffer),
                bind_group_buffer_entry(3, &mesh.vertices_buffer)
            ]
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(gpu_pipeline);
        pass.set_bind_group(0, &mesh_group, &[]);
        pass.dispatch_workgroups(mesh_info.workgroups_x, mesh_info.workgroups_y, 1);
        drop(pass);
        gpu.queue.submit(Some(encoder.finish()));
        mesh_view
    }

    fn mesh_info(&self, longitudes: u16, latitudes: u16) -> MeshInfo {
        let latitudes_u32 = latitudes as u32;
        let longitudes_u32 = longitudes.max(latitudes) as u32;
        MeshInfo {
            size: MeshSize {
                indices_count: 6 * longitudes_u32 * latitudes_u32,
                vertices_count: (longitudes_u32 + 1) * (latitudes_u32 + 1),
            },
            workgroups_x: (longitudes_u32 + 1).div_ceil(8),
            workgroups_y: (latitudes_u32 + 1).div_ceil(8),
        }
    }

}