use std::mem::size_of;

use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

pub struct CameraDescriptor {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct Camera<'a> {
    position: Point3<f32>,
    target: Point3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    uniform_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
    queue: &'a wgpu::Queue,
}

impl<'a> Camera<'a> {
    pub fn new(
        descriptor: CameraDescriptor,
        device: &wgpu::Device,
        queue: &'a wgpu::Queue,
    ) -> Self {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera uniform buffer"),
            size: size_of::<f32>() as u64 * 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform buffer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform buffer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let camera = Camera {
            position: descriptor.position,
            target: descriptor.target,
            aspect: descriptor.aspect,
            fovy: descriptor.fovy,
            znear: descriptor.znear,
            zfar: descriptor.zfar,
            uniform_buffer,
            bind_group_layout,
            uniform_bind_group,
            queue,
        };

        camera.update_camera_buffer();

        camera
    }

    fn get_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_matrix = Matrix4::look_at_rh(self.position, self.target, Vector3::unit_y());
        let projection_matrix = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix
    }

    fn update_camera_buffer(&self) {
        let camera_uniform_data: [[f32; 4]; 4] = self.get_view_projection_matrix().into();

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&camera_uniform_data),
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
        self.update_camera_buffer();
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        self.update_camera_buffer();
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
