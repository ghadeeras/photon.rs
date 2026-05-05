use crate::wgpu::gpu;
use std::sync::Arc;

pub struct Tracer {
    gpu: Arc<gpu::GPU>,
    gpu_pipeline: wgpu::RenderPipeline,
    format: wgpu::TextureFormat,
}

impl Tracer {

    pub fn new(gpu: Arc<gpu::GPU>, format: wgpu::TextureFormat) -> Self {
        let tracing_shader = gpu.device.create_shader_module(wgpu::include_wgsl!("./tracer.wgsl"));
        let gpu_pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &tracing_shader,
                entry_point: Some("v_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &tracing_shader,
                entry_point: Some("f_main"),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::COLOR,
                    })
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                ..Default::default()
            },
            multisample: Default::default(),
            multiview_mask: None,
            depth_stencil: None,
            cache: None
        });
        Self {
            gpu,
            gpu_pipeline,
            format,
        }
    }

    pub fn render(&self, texture: &wgpu::Texture) {
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
        pass.set_pipeline(&self.gpu_pipeline);
        pass.draw(0..3, 0..1);
        drop(pass);
        self.gpu.queue.submit(Some(encoder.finish()));
    }

}

