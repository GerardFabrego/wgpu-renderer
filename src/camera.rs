use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

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
    direction: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera{
    pub fn new(
        descriptor: CameraDescriptor,
    ) -> Self {
        Self  {
            position: descriptor.position,
            direction: descriptor.direction,
            aspect: descriptor.aspect,
            fovy: descriptor.fovy,
            znear: descriptor.znear,
            zfar: descriptor.zfar,
        }
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_matrix = Matrix4::look_to_rh(self.position, self.direction, Vector3::unit_y());
        let projection_matrix = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix
    }

    pub fn get_position(&self) -> cgmath::Point3<f32> {
        self.position
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }

    pub fn rotate(&mut self, y_rotation: f32, x_rotation: f32) {
        let rotation_speed = 0.01;
        let x_rotation = x_rotation * rotation_speed;
        let y_rotation = y_rotation * rotation_speed;
        
        let cos_x = x_rotation.cos();
        let sin_x = x_rotation.sin();

        let cos_y = y_rotation.cos();
        let sin_y = y_rotation.sin();

        #[rustfmt::skip]
        let x_rotation_matrix = cgmath::Matrix3::new(
            1.0, 0.0, 0.0, 
            0.0, cos_x, -sin_x, 
            0.0, sin_x, cos_x
        );

        #[rustfmt::skip]
        let y_rotation_matrix = cgmath::Matrix3::new(
            cos_y, 0.0, sin_y, 
            0.0, 1.0, 0.0, 
            -sin_y, 0.0, cos_y
        );

        self.direction = x_rotation_matrix * y_rotation_matrix * self.direction;
    }
    
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);