use std::sync::Arc;

pub struct GPU {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl GPU {

    pub async fn new(gpu_adapter: Arc<wgpu::Adapter>) -> GPU {
        let (gpu_device, gpu_queue) = gpu_adapter.request_device(&Default::default()).await.unwrap();
        Self {
            device: Arc::new(gpu_device), queue: Arc::new(gpu_queue)
        }
    }

}