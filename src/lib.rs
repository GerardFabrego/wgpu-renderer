mod camera;
mod primitives;
mod texture;
mod vertex;
mod window;

use camera::{Camera, CameraDescriptor};
use cgmath::Vector3;
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

    let mut camera = Camera::new(camera_descriptor, &device, &queue);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render pipeline layout"),
        bind_group_layouts: &[&object.bind_group_layout, &camera.bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Cube::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    // Event loop
    window.run(|event, window_commands| match event {
        Event::Resize(width, height) => {
            camera.resize(width, height);
            config.width = width;
            config.height = height;
            surface.configure(&device, &config);
        }
        Event::Draw => {
            let current_texture = surface.get_current_texture().unwrap();
            let view = current_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.8,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_bind_group(0, &object.bind_group, &[]);
            render_pass.set_bind_group(1, &camera.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, object.vertex_buffer.slice(..));
            render_pass.set_index_buffer(object.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..object.index_count as u32, 0, 0..2);

            drop(render_pass);

            let command_buffer = encoder.finish();

            queue.submit(std::iter::once(command_buffer));
            current_texture.present();
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
