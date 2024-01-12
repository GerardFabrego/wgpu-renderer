use crate::texture;

pub struct Material {
    pub name: String,
    pub texture: texture::Texture,
    // pub bind_group: wgpu::BindGroup,
}
