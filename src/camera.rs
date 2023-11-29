use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    target: Point3<f32>,
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            position,
            target,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }

    fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, Vector3::unit_y())
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar)
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * self.get_projection_matrix() * self.get_view_matrix()
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
