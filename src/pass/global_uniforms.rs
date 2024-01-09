use crate::camera::Camera;

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
