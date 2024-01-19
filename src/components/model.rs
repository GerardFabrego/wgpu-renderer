use crate::utils::load_texture;

use super::{geometry::Material, Mesh};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn cube(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_path: &str,
    ) -> anyhow::Result<Self> {
        let texture = load_texture(texture_path, device, queue)?;

        let materials = vec![Material {
            name: String::from("Cube material"),
            texture,
        }];

        let meshes = vec![Mesh::create_cube(device, queue, 0)?];

        Ok(Self { meshes, materials })
    }
}
