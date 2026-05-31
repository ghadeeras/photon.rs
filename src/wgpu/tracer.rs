use crate::wgpu::app::{Renderer, RendererFactory};
use crate::wgpu::gpu::GPU;
use crate::wgpu::primitive_assembly::{PrimitiveAssembly, Triangles};
use std::time::Duration;
use wgpu::{RenderPipeline, Texture, TextureFormat};

pub struct TracerFactory;
pub struct Tracer {
    gpu: GPU,
    gpu_pipeline: wgpu::RenderPipeline,
    format: TextureFormat,
    triangles: Triangles,
    elapsed_time: Duration,
}

impl RendererFactory for TracerFactory {

    type Output = Tracer;

    fn new_renderer(&self, gpu: GPU, format: TextureFormat) -> Self::Output {
        Tracer::new(gpu, format)
    }

}

impl Tracer {

    pub fn new(gpu: GPU, format: TextureFormat) -> Self {
        let tracing_shader = gpu.device.create_shader_module(wgpu::include_wgsl!("./tracer.wgsl"));
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
        let triangles = Self::triangles(&gpu, &gpu_pipeline);
        Self {
            gpu,
            gpu_pipeline,
            format,
            triangles,
            elapsed_time: Duration::default(),
        }
    }

    fn triangles(gpu: &GPU, gpu_pipeline: &RenderPipeline) -> Triangles {
        let triangles = PrimitiveAssembly::new_triangles(&gpu);
        let triangles_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &gpu_pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &triangles.triangles_buffer,
                    size: None,
                    offset: 0
                })
            }, wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &triangles.vertices_buffer,
                    size: None,
                    offset: 0
                })
            }]
        });
        Triangles {
            triangles_group,
            triangles_buffer: triangles.triangles_buffer,
            vertices_buffer: triangles.vertices_buffer,
        }
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

    fn render(&self, texture: &Texture) {
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
        pass.set_bind_group(0, &self.triangles.triangles_group, &[]);
        pass.draw(0..3, t..(t + 1));
        drop(pass);
        self.gpu.queue.submit(Some(encoder.finish()));
    }

}

