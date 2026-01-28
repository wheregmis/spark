use std::time::Instant;

use spark_core::{pipeline::Pipeline, wgpu_init::init_wgpu};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct SceneUniform {
    color: [f32; 4],
}

struct App {
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    surface_state: Option<spark_core::wgpu_init::SurfaceState<'static>>, // demo uses leak to satisfy lifetimes
    pipeline: Option<Pipeline<SceneUniform>>,
    start: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self { device: None, queue: None, surface_state: None, pipeline: None, start: Instant::now() }
    }
}

impl winit::application::ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(winit::window::WindowAttributes::default().with_title("spark triangle"))
            .expect("create window");
        let window_leaked: &'static mut Box<dyn winit::window::Window> = Box::leak(Box::new(window));
        let window: &'static dyn winit::window::Window = &**window_leaked;

        let (device, queue, surface_state) = pollster::block_on(init_wgpu(window));
        let shader_src = include_str!("../../../assets/shaders/basic.wgsl");
        let pipeline: Pipeline<SceneUniform> = Pipeline::new(
            &device,
            shader_src,
            "vs_main",
            "fs_main",
            wgpu::TextureFormat::from(surface_state.config.format),
        );

        self.device = Some(device);
        self.queue = Some(queue);
        self.surface_state = Some(surface_state);
        self.pipeline = Some(pipeline);
        self.start = Instant::now();
    }

    fn window_event(&mut self, _event_loop: &dyn winit::event_loop::ActiveEventLoop, _id: winit::window::WindowId, event: winit::event::WindowEvent) {
        if let (Some(device), Some(surface_state)) = (self.device.as_ref(), self.surface_state.as_mut()) {
            match event {
                winit::event::WindowEvent::SurfaceResized(size) => {
                    if size.width > 0 && size.height > 0 {
                        surface_state.resize(device, size.width, size.height);
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        if let (Some(device), Some(queue), Some(surface_state), Some(pipeline)) =
            (self.device.as_ref(), self.queue.as_ref(), self.surface_state.as_mut(), self.pipeline.as_mut())
        {
            let t = self.start.elapsed().as_secs_f32();
            let color = [t.sin() * 0.5 + 0.5, t.cos() * 0.5 + 0.5, 0.3, 1.0];
            pipeline.update_uniforms(queue, &SceneUniform { color });

            let frame = match surface_state.surface.get_current_texture() {
                Ok(frame) => frame,
                Err(_) => {
                    surface_state.reconfigure(device);
                    surface_state.surface.get_current_texture().unwrap()
                }
            };
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("main_encoder") });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("clear+draw"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.set_pipeline(&pipeline.pipeline);
                rpass.set_bind_group(0, &pipeline.bind_group, &[]);
                rpass.draw(0..3, 0..1);
            }
            queue.submit(Some(encoder.finish()));
            frame.present();
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let app: &'static mut App = Box::leak(Box::new(App::default()));
    event_loop.run_app(app).unwrap();
}
