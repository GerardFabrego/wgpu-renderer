use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};

mod camera_controller;

pub use self::camera_controller::CameraController;

pub struct CameraDescriptor {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct Camera {
    position: Point3<f32>,
    // direction: Vector3<f32>,
    pitch: Rad<f32>,
    yaw: Rad<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(descriptor: CameraDescriptor) -> Self {
        Self {
            position: descriptor.position,
            pitch: Rad(0.0),
            yaw: Deg(-90.0).into(),
            aspect: descriptor.aspect,
            fovy: descriptor.fovy,
            znear: descriptor.znear,
            zfar: descriptor.zfar,
        }
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        let view_matrix = Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        );
        let projection_matrix = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix
    }

    pub fn get_position(&self) -> cgmath::Point3<f32> {
        self.position
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
