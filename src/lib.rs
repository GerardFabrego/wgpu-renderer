mod camera;
mod pass;
mod primitives;
mod texture;
mod vertex;
mod window;

use camera::{Camera, CameraDescriptor};
use cgmath::Vector3;
use pass::{Pass, PhongPass};
use primitives::Cube;
use window::{Event, Window};

struct GraphicsContext {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsContext {
    async fn new(window: &Window) -> GraphicsContext {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            flags: Default::default(),
            gles_minor_version: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window.window) }.unwrap();

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

        let (width, height) = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        GraphicsContext {
            surface,
            config,
            device,
            queue,
        }
    }
}

pub async fn run() {
    let window = Window::new();

    let GraphicsContext {
        device,
        queue,
        mut config,
        surface,
    }: GraphicsContext = GraphicsContext::new(&window).await;

    let object = Cube::new(&device, &queue);

    let camera_descriptor = CameraDescriptor {
        position: (0.0, 2.0, 4.0).into(),
        direction: (0.0, 0.0, -1.0).into(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera = Camera::new(camera_descriptor);

    let mut pass = PhongPass::new(&device, &queue, &config);

    // Event loop
    window.run(|event, window_commands| match event {
        Event::Resize(width, height) => {
            camera.resize(width, height);
            config.width = width;
            config.height = height;
            surface.configure(&device, &config);
        }
        Event::Draw => {
            pass.draw(&surface, &device, &queue, &object, &camera);
        }
        Event::KeyboardInput(key) => match key {
            window::Key::Left | window::Key::Letter('a') => {
                camera.translate(Vector3::new(-1.0, 0.0, 0.0))
            }
            window::Key::Right | window::Key::Letter('d') => {
                camera.translate(Vector3::new(1.0, 0.0, 0.0))
            }
            window::Key::Up | window::Key::Letter('w') => {
                camera.translate(Vector3::new(0.0, 1.0, 0.0))
            }
            window::Key::Down | window::Key::Letter('s') => {
                camera.translate(Vector3::new(0.0, -1.0, 0.0))
            }
            window::Key::Escape => window_commands.exit(),
            _ => {}
        },
        Event::MouseMove(y, x) => camera.rotate(y, x),
    });
}
