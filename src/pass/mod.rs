use crate::{camera::Camera, entity::Entity};

mod phong;
mod uniform_pool;

pub use self::phong::PhongPass;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl Globals {
    pub fn from(camera: &Camera) -> Self {
        let camera_position = camera.get_position();
        Self {
            view_position: [camera_position.x, camera_position.y, camera_position.z, 0.0],
            view_proj: camera.get_view_projection_matrix().into(),
        }
    }
}
pub trait Pass {
    fn draw(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        entities: &Vec<Entity>,
        camera: &Camera,
    );
}
