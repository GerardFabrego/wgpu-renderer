use crate::mesh::Mesh;

mod phong;

pub use self::phong::PhongPass;

pub trait Pass {
    fn draw(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        object: &Mesh,
    );
}
