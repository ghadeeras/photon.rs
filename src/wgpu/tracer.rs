use crate::wgpu::app::{Renderer, RendererFactory};
use crate::wgpu::bind_group_entry;
use crate::wgpu::gpu::GPU;
use crate::wgpu::meshes::{MeshGenerator, Meshable};
use crate::wgpu::primitive_assembly::PrimitiveAssembly;
use std::time::Duration;

pub struct TracerFactory<G: MeshGenerator, M: Meshable<Generator=G>> {
    pub meshable: M,
    pub params: G::Params,
}

pub struct Tracer {
    gpu: GPU,
    gpu_pipeline: wgpu::RenderPipeline,
    format: wgpu::TextureFormat,
    triangles_group: wgpu::BindGroup,
    elapsed_time: Duration,
}

pub struct Triangles {
    pub triangles_buffer: wgpu::Buffer,
    pub vertices_buffer: wgpu::Buffer,
}

impl<G: MeshGenerator, M: Meshable<Generator=G>> RendererFactory for TracerFactory<G, M> {

    type Output = Tracer;

    fn new_renderer(&self, gpu: GPU, format: wgpu::TextureFormat) -> Self::Output {
        let assembly = PrimitiveAssembly::new(&gpu);
        let generator = self.meshable.generator(&gpu);
        let mesh = generator.mesh(&self.params);
        let triangles_buffer = assembly.triangles(&mesh);
        Tracer::new(gpu, format, Triangles {
            triangles_buffer,
            vertices_buffer: mesh.vertices_buffer,
        })
    }

}

impl Tracer {

    pub fn new(gpu: GPU, format: wgpu::TextureFormat, triangles: Triangles) -> Self {
        let tracing_shader = gpu.device.create_shader_module(wgpu::include_wgsl!("./shaders/tracer.wgsl"));
        let gpu_pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &tracing_shader,
                entry_point: None,
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &tracing_shader,
                entry_point: None,
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::all(),
                    })
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            multisample: Default::default(),
            multiview_mask: None,
            depth_stencil: None,
            cache: None
        });
        let triangles_group = Self::triangles_group(&gpu, &gpu_pipeline, triangles);
        Self {
            gpu,
            gpu_pipeline,
            format,
            triangles_group,
            elapsed_time: Duration::default(),
        }
    }

    fn triangles_group(gpu: &GPU, gpu_pipeline: &wgpu::RenderPipeline, triangles: Triangles) -> wgpu::BindGroup {
        let triangles_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &gpu_pipeline.get_bind_group_layout(0),
            entries: &[
                bind_group_entry(0, &triangles.triangles_buffer),
                bind_group_entry(1, &triangles.vertices_buffer),
            ]
        });
        triangles_group
    }

    pub fn gpu(&self) -> &GPU {
        &self.gpu
    }

}

impl Renderer for Tracer {

    fn gpu(&self) -> &GPU {
        &self.gpu
    }

    fn animate(&mut self, elapses_time: Duration) {
        self.elapsed_time = elapses_time;
    }

    fn render(&self, texture: &wgpu::Texture) {
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &texture.create_view(&wgpu::TextureViewDescriptor {
                        format: Some(self.format),
                        ..Default::default()
                    }),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })
            ],
            ..Default::default()
        });
        let t = (self.elapsed_time.as_millis() & 0x7FFFFFFF) as u32;
        pass.set_pipeline(&self.gpu_pipeline);
        pass.set_bind_group(0, &self.triangles_group, &[]);
        pass.draw(0..3, t..(t + 1));
        drop(pass);
        self.gpu.queue.submit(Some(encoder.finish()));
    }

}

