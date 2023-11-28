use std::mem::size_of;

use cgmath::Vector3;
use winit::keyboard::KeyCode;

use crate::{camera::Camera, vertex::Vertex};

pub struct Scene {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    camera: Camera,
    camera_uniform_bind_group: wgpu::BindGroup,
    camera_uniform_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

fn create_cube_data() -> ([Vertex; 24], [u32; 36]) {
    #[rustfmt::skip]
    let vertices = [

        Vertex {position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex {position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex {position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] },

        Vertex {position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex {position: [ 1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex {position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex {position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] },

        Vertex {position: [ 1.0, -1.0, -1.0], color: [0.0, 0.0, 1.0] },
        Vertex {position: [ 1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
        Vertex {position: [ 1.0, -1.0,  1.0], color: [0.0, 0.0, 1.0] },

        Vertex {position: [-1.0, -1.0,  1.0], color: [1.0, 1.0, 0.0] },
        Vertex {position: [-1.0,  1.0,  1.0], color: [1.0, 1.0, 0.0] },
        Vertex {position: [-1.0,  1.0, -1.0], color: [1.0, 1.0, 0.0] },
        Vertex {position: [-1.0, -1.0, -1.0], color: [1.0, 1.0, 0.0] },

        Vertex {position: [ 1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] },
        Vertex {position: [-1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] },
        Vertex {position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] },

        Vertex {position: [ 1.0, -1.0,  1.0], color: [0.0, 1.0, 1.0] },
        Vertex {position: [-1.0, -1.0,  1.0], color: [0.0, 1.0, 1.0] },
        Vertex {position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] },
        Vertex {position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] },
    ];

    let indices = [
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertices, indices)
}

impl Scene {
    pub fn init(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let (vertices, indices) = create_cube_data();

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
            size: (size_of::<u32>() * indices.len()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));

        let camera = Camera::new(
            (0.0, 2.0, 4.0).into(),
            (0.0, 0.0, 0.0).into(),
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            100.0,
        );

        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera uniform buffer"),
            size: size_of::<f32>() as u64 * 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
            camera,
            camera_uniform_bind_group,
            camera_uniform_buffer,
            render_pipeline,
        }
    }

    pub fn render(&self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let camera_uniform_data: [[f32; 4]; 4] = self.camera.get_view_projection_matrix().into();

        queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&camera_uniform_data),
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count as u32, 0, 0..2);

        drop(render_pass);

        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
    }

    pub fn move_camera(&mut self, code: KeyCode) {
        match code {
            KeyCode::KeyA => self.camera.position += Vector3::new(1.0, 0.0, 0.0),
            KeyCode::KeyD => self.camera.position += Vector3::new(-1.0, 0.0, 0.0),
            _ => {}
        }
    }
}
