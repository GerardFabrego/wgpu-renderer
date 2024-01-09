use crate::entity::Entity;

mod global_uniforms;
mod phong;

pub use self::phong::PhongPass;

pub trait Pass {
    fn draw(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        entity: &Entity,
    );
}
