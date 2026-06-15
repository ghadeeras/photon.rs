#[derive(Clone)]
pub struct GPU {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPU {

    pub async fn new(gpu_instance: wgpu::Instance, compatible_surface: Option<&wgpu::Surface<'static>>) -> anyhow::Result<GPU> {
        let gpu_adapter = gpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface,
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }).await?;
        let (gpu_device, gpu_queue) = gpu_adapter.request_device(&Default::default()).await?;
        Ok(Self {
            instance: gpu_instance, adapter: gpu_adapter, device: gpu_device, queue: gpu_queue
        })
    }

}