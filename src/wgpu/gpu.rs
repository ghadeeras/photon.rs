use std::rc::Rc;

pub struct GPU {
    _adapter: Rc<wgpu::Adapter>,
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
}

impl GPU {

    pub fn new_instance() -> wgpu::Instance {
        wgpu::Instance::new(Default::default())
    }

    pub async fn new_adapter<'s>(gpu_instance: &wgpu::Instance, surface: Option<&wgpu::Surface<'s>>) -> wgpu::Adapter {
        gpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: surface,
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }).await.unwrap()
    }

    pub async fn new(gpu_adapter: Rc<wgpu::Adapter>) -> GPU {
        let (gpu_device, gpu_queue) = gpu_adapter.request_device(&Default::default(), None).await.unwrap();
        Self {
            _adapter: gpu_adapter, device: Rc::new(gpu_device), queue: Rc::new(gpu_queue)
        }
    }

    pub fn device(&self) -> Rc<wgpu::Device> {
        self.device.clone()
    }

    pub fn queue(&self) -> Rc<wgpu::Queue> {
        self.queue.clone()
    }

}