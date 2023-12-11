use std::mem::size_of;

use wgpu::{
    ImageCopyTexture, ImageDataLayout, Origin3d, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages,
};

use crate::vertex::Vertex;

fn create_cube_data() -> ([Vertex; 24], [u32; 36]) {
    #[rustfmt::skip]
    let vertices = [

        Vertex {position: [-1.0, -1.0,  1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [ 1.0, -1.0,  1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [-1.0,  1.0,  1.0], text_coords: [0.0, 0.0] },

        Vertex {position: [-1.0,  1.0, -1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [ 1.0,  1.0, -1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [ 1.0, -1.0, -1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [-1.0, -1.0, -1.0], text_coords: [0.0, 0.0] },

        Vertex {position: [ 1.0, -1.0, -1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [ 1.0,  1.0, -1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [ 1.0, -1.0,  1.0], text_coords: [0.0, 0.0] },

        Vertex {position: [-1.0, -1.0,  1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [-1.0,  1.0,  1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [-1.0,  1.0, -1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [-1.0, -1.0, -1.0], text_coords: [0.0, 0.0] },

        Vertex {position: [ 1.0,  1.0, -1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [-1.0,  1.0, -1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [-1.0,  1.0,  1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [ 1.0,  1.0,  1.0], text_coords: [0.0, 0.0] },

        Vertex {position: [ 1.0, -1.0,  1.0], text_coords: [0.0, 1.0] },
        Vertex {position: [-1.0, -1.0,  1.0], text_coords: [1.0, 1.0] },
        Vertex {position: [-1.0, -1.0, -1.0], text_coords: [1.0, 0.0] },
        Vertex {position: [ 1.0, -1.0, -1.0], text_coords: [0.0, 0.0] },
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

pub struct Object {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: usize,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Object {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let diffuse_bytes = include_bytes!("../res/textures/test.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let (vertices, indices) = create_cube_data();

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

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
            bind_group_layout,
            bind_group,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 3 * size_of::<f32>() as u64,
                    shader_location: 1,
                },
            ],
        }
    }
}
