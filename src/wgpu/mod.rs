use wgpu::Buffer;

pub mod app;
pub mod canvas;
pub mod gpu;
pub mod tracer;
pub mod primitive_assembly;
pub mod geometry;
pub mod data;

#[allow(clippy::needless_lifetimes)]
pub fn bind_group_entry<'a>(binding: u32, buffer: &'a Buffer) -> wgpu::BindGroupEntry<'a> {
    wgpu::BindGroupEntry {
        binding,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &&buffer,
            size: None,
            offset: 0
        })
    }
}