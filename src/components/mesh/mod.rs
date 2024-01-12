mod primitives;
mod vertex;

use crate::texture::Texture;

pub use self::vertex::Vertex;

pub struct Mesh {
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
}
