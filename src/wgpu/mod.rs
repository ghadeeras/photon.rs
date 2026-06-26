use crate::wgpu::data::{Data, Writable};
use crate::wgpu::gpu::GPU;
use wgpu::wgt::BufferDescriptor;
use wgpu::Buffer;

pub mod app;
pub mod canvas;
pub mod gpu;
pub mod tracer;
pub mod primitive_assembly;
pub mod meshes;
pub mod data;

#[allow(clippy::needless_lifetimes)]
pub fn bind_group_buffer_entry<'a>(binding: u32, buffer: &'a Buffer) -> wgpu::BindGroupEntry<'a> {
    bind_group_buffer_view_entry(binding, buffer, 0, 0)
}

#[allow(clippy::needless_lifetimes)]
pub fn bind_group_buffer_view_entry<'a>(binding: u32, buffer: &'a Buffer, offset: usize, size: usize) -> wgpu::BindGroupEntry<'a> {
    wgpu::BindGroupEntry {
        binding,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &&buffer,
            size: wgpu::BufferSize::new(size as u64),
            offset: offset as u64,
        })
    }
}

pub fn uniform_buffer(gpu: &GPU, label: &str, size: usize) -> Buffer {
    new_buffer(gpu, label, wgpu::BufferUsages::UNIFORM, size, false)
}

pub fn initialized_uniform_buffer<D: Data>(gpu: &GPU, label: &str, data: &D) -> Buffer {
    initialized_buffer(gpu, &label, wgpu::BufferUsages::UNIFORM, data)
}

pub fn storage_buffer(gpu: &GPU, label: &str, size: usize) -> Buffer {
    new_buffer(gpu, label, wgpu::BufferUsages::STORAGE, size, false)
}

pub fn initialized_storage_buffer<D: Data>(gpu: &GPU, label: &str, data: &D) -> Buffer {
    initialized_buffer(gpu, &label, wgpu::BufferUsages::STORAGE, data)
}

fn initialized_buffer<D: Data>(gpu: &GPU, label: &&str, usages: wgpu::BufferUsages, data: &D) -> Buffer {
    let buffer = new_buffer(gpu, &label, usages, D::padded_size(), true);
    let mut range = buffer.get_mapped_range_mut(..);
    Writable::new(&mut range).write(data);
    drop(range);
    buffer.unmap();
    buffer
}

fn new_buffer(gpu: &GPU, label: &str, usages: wgpu::BufferUsages, size: usize, mapped_at_creation: bool) -> Buffer {
    gpu.device.create_buffer(&BufferDescriptor {
        label: Some(label),
        usage: wgpu::BufferUsages::COPY_SRC.union(wgpu::BufferUsages::COPY_DST).union(usages),
        mapped_at_creation,
        size: size as u64,
    })
}
