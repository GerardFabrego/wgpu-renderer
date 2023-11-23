mod camera;
mod vertex;

use std::mem::size_of;

use camera::Camera;
use log::info;
use vertex::Vertex;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

async fn wgpu_init(
    window: &Window,
) -> (
    wgpu::Surface,
    wgpu::SurfaceConfiguration,
    wgpu::Device,
    wgpu::Queue,
) {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: Default::default(),
        flags: Default::default(),
        gles_minor_version: Default::default(),
    });

    let surface = unsafe { instance.create_surface(window) }.unwrap();

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

    return (surface, config, device, queue);
}

fn main() {
    // Window creation
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window: Window = WindowBuilder::new()
        .build(&event_loop)
        .expect("There was an error when building the window");

    // WGPU context creation
    let (surface, mut config, device, queue) = pollster::block_on(wgpu_init(&window));

    #[rustfmt::skip]
    let vertices: [Vertex; 4] = [
        Vertex {position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex {position: [ 0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex {position: [ 0.5,  0.5, 0.0], color: [0.0, 0.0, 1.0] },
        Vertex {position: [-0.5,  0.5, 0.0], color: [0.0, 1.0, 0.0] },
    ];

    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 3 * size_of::<f32>() as u64,
                shader_location: 1,
            },
        ],
    };

    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex buffer"),
        size: (size_of::<Vertex>() * vertices.len()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Index buffer"),
        size: (size_of::<u32>() * 6) as u64,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));

    let camera = Camera::new(
        (0.0, 0.0, 1.0).into(),
        (0.0, 0.0, 0.0).into(),
        config.width as f32 / config.height as f32,
        45.0,
        0.1,
        100.0,
    );

    let camera_uniform_data: [[f32; 4]; 4] = camera.get_view_projection_matrix().into();

    println!("HERE: {:?}", camera_uniform_data);

    let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Camera uniform buffer"),
        size: size_of::<f32>() as u64 * 16,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(
        &camera_uniform_buffer,
        0,
        bytemuck::cast_slice(&camera_uniform_data),
    );

    let camera_uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform buffer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let camera_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Uniform buffer bind group"),
        layout: &camera_uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_uniform_buffer.as_entire_binding(),
        }],
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render pipeline layout"),
        bind_group_layouts: &[&camera_uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
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
    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => elwt.exit(),
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

                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Render encoder"),
                            });

                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                        render_pass.set_bind_group(0, &camera_uniform_bind_group, &[]);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..6, 0, 0..2);

                        drop(render_pass);

                        let command_buffer = encoder.finish();

                        queue.submit(std::iter::once(command_buffer));
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
