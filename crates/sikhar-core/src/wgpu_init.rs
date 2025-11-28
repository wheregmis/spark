use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

pub struct SurfaceState<'a> {
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

pub async fn init_wgpu<'a>(window: &'a dyn Window) -> (Device, Queue, SurfaceState<'a>) {
    let size = window.surface_size();

    // On web, prefer WebGPU. On native, use primary backends.
    #[cfg(target_arch = "wasm32")]
    let backends = Backends::BROWSER_WEBGPU | Backends::GL;
    #[cfg(not(target_arch = "wasm32"))]
    let backends = Backends::PRIMARY;

    let instance = Instance::new(&InstanceDescriptor {
        backends,
        ..Default::default()
    });
    let surface = instance.create_surface(window).expect("create surface");

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("adapter");

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: Default::default(),
                experimental_features: Default::default(),
                trace: Default::default(),
            },
        )
        .await
        .expect("device");

    let caps = surface.get_capabilities(&adapter);
    let format = caps.formats[0];

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width.max(1),
        height: size.height.max(1),
        present_mode: caps.present_modes[0],
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let mut state = SurfaceState { surface, config, size };
    state.reconfigure(&device);

    (device, queue, state)
}

impl<'a> SurfaceState<'a> {
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.size = PhysicalSize::new(width, height);
        self.reconfigure(device);
    }

    pub fn reconfigure(&mut self, device: &Device) {
        if self.size.width > 0 && self.size.height > 0 {
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(device, &self.config);
        }
    }
}
