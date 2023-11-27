mod camera;
mod scene;
mod vertex;

use log::info;
use scene::Scene;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

struct Setup {
    event_loop: EventLoop<()>,
    _window: Window,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn wgpu_init() -> Setup {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window: Window = WindowBuilder::new()
        .build(&event_loop)
        .expect("There was an error when building the window");

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: Default::default(),
        flags: Default::default(),
        gles_minor_version: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let surface_capabilities = surface.get_capabilities(&adapter);

    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_capabilities.formats[0]);

    let size = window.inner_size();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    Setup {
        event_loop,
        _window: window,
        surface,
        config,
        device,
        queue,
    }
}

fn run(
    Setup {
        surface,
        mut config,
        device,
        queue,
        event_loop,
        ..
    }: Setup,
) {
    let scene = Scene::init(&device, &queue, &config);

    // Event loop
    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let PhysicalKey::Code(code) = event.physical_key {
                            match code {
                                KeyCode::Escape => elwt.exit(),
                                KeyCode::KeyA => {}
                                _ => (),
                            }
                        }
                    }
                    WindowEvent::Resized(new_size) => {
                        config.height = new_size.height;
                        config.width = new_size.width;
                        surface.configure(&device, &config);
                    }
                    WindowEvent::RedrawRequested => {
                        let current_texture = surface.get_current_texture().unwrap();
                        let view = current_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        scene.render(&view, &device, &queue);
                        current_texture.present();
                    }
                    _ => {}
                };
            }
        })
        .unwrap_or_else(|error| {
            info!("Problem on the window event loop: {:?}", error);
        });
}

fn main() {
    let setup = pollster::block_on(wgpu_init());
    run(setup);
}
