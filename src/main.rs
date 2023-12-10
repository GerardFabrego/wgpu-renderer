mod camera;
mod object;
mod scene;
mod vertex;
mod window;

use scene::Scene;
use window::{Event, Window};

struct Setup {
    window: Window,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn wgpu_init() -> Setup {
    let window = Window::new();

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

    Setup {
        window,
        surface,
        config,
        device,
        queue,
    }
}

fn run(
    Setup {
        surface,
        window,
        mut config,
        device,
        queue,
        ..
    }: Setup,
) {
    let mut scene = Scene::init(&device, &queue, &config);

    // Event loop
    window.run(|event| match event {
        Event::Resize(width, height) => {
            scene.resize(width, height);
            config.width = width;
            config.height = height;
            surface.configure(&device, &config);
        }
        Event::Draw => {
            let current_texture = surface.get_current_texture().unwrap();
            let view = current_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            scene.render(&view);
            current_texture.present();
        }
        Event::KeyboardInput(key) => scene.camera.move_camera(key),
    });
}

fn main() {
    let setup = pollster::block_on(wgpu_init());
    run(setup);
}
