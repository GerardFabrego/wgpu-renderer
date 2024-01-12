use std::{collections::HashMap, mem::size_of};

use crate::{
    camera::Camera,
    components::{TransformRaw, Vertex},
    entity::Entity,
    texture::Texture,
};

use super::{uniform_pool::UniformPool, Globals};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Locals {
    pub m_matrix: TransformRaw,
}

pub struct PhongPass {
    global_uniform_buffer: wgpu::Buffer,
    global_bind_group: wgpu::BindGroup,

    local_bind_group_layout: wgpu::BindGroupLayout,
    local_uniforms_pool: UniformPool,
    local_bind_groups: HashMap<usize, wgpu::BindGroup>,

    pub depth_texture: Texture,

    pipeline: wgpu::RenderPipeline,
}

impl PhongPass {
    pub(crate) fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> PhongPass {
        // GLOBAL UNIFORMS
        let global_size = size_of::<Globals>() as wgpu::BufferAddress;

        let global_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Phong Globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(global_size),
                    },
                    count: None,
                }],
            });

        let global_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Phong Globals buffer"),
            size: global_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Phong Globals bind group"),
            layout: &global_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: global_uniform_buffer.as_entire_binding(),
            }],
        });

        // LOCAL UNIFORMS
        let local_size = size_of::<Locals>() as wgpu::BufferAddress;
        let local_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("[Phong] Locals"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(local_size),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let local_uniforms_pool = UniformPool::new("Local uniforms pool", local_size);

        // DEPTH TEXTURE
        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[&global_bind_group_layout, &local_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        PhongPass {
            global_uniform_buffer,
            global_bind_group,

            local_bind_group_layout,
            local_uniforms_pool,
            local_bind_groups: Default::default(),

            depth_texture,

            pipeline,
        }
    }
}

impl super::Pass for PhongPass {
    fn draw(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        entities: &Vec<Entity>,
        camera: &Camera,
    ) {
        queue.write_buffer(
            &self.global_uniform_buffer,
            0,
            bytemuck::cast_slice(&[Globals::from(camera)]),
        );

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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.global_bind_group, &[]);

        if self.local_uniforms_pool.buffers.len() < entities.len() {
            self.local_uniforms_pool
                .alloc_buffers(entities.len(), device);
        }

        for (index, entity) in entities.iter().enumerate() {
            let local_buffer = &self.local_uniforms_pool.buffers[index];

            queue.write_buffer(
                local_buffer,
                0,
                bytemuck::cast_slice(&[Locals {
                    m_matrix: TransformRaw::from(&entity.transform),
                }]),
            );

            self.local_bind_groups.entry(index).or_insert_with(|| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("[Phong] Locals"),
                    layout: &self.local_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: local_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                &entity.mesh.get_texture().view,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Sampler(
                                &entity.mesh.get_texture().sampler,
                            ),
                        },
                    ],
                })
            });
        }

        for (index, entity) in entities.iter().enumerate() {
            render_pass.set_bind_group(1, &self.local_bind_groups[&index], &[]);
            render_pass.set_vertex_buffer(0, entity.mesh.get_vertex_buffer().slice(..));
            render_pass.set_index_buffer(
                entity.mesh.get_index_buffer().slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..entity.mesh.get_index_count() as u32, 0, 0..2);
        }

        drop(render_pass);

        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        current_texture.present();
    }
}
