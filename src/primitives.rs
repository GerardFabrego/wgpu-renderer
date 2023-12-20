use std::mem::size_of;

use crate::{texture::Texture, utils::load_texture, vertex::Vertex};

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

pub struct Cube {
    pub transform: Transform,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    texture: Texture,
}

impl Cube {
    pub async fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        transform: Transform,
    ) -> anyhow::Result<Self> {
        let texture = load_texture("textures/test.png", device, queue).await?;

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

        Ok(Self {
            transform,
            texture,
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
        })
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }
    pub fn get_index_count(&self) -> usize {
        self.index_count
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

pub struct Transform {
    pub position: cgmath::Vector3<f32>,
    pub scale: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    m_matrix: [[f32; 4]; 4],
}

impl TransformRaw {
    pub fn from(transform: &Transform) -> Self {
        Self {
            m_matrix: (cgmath::Matrix4::from_translation(transform.position)
                * cgmath::Matrix4::from_nonuniform_scale(
                    transform.scale.x,
                    transform.scale.y,
                    transform.scale.z,
                )
                * cgmath::Matrix4::from(transform.rotation))
            .into(),
        }
    }
}
