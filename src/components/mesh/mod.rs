mod primitives;
mod vertex;

use std::mem::size_of;

use crate::texture::Texture;

use vertex::Vertex;

pub struct Mesh {
    // pub transform: Transform,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    texture: Texture,
}

impl Mesh {
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
