use std::mem::size_of;

use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

use crate::window::Key;

pub struct CameraDescriptor {
    pub position: Point3<f32>,
    target: Point3<f32>,
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl CameraDescriptor {
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

    pub fn move_camera(&mut self, code: Key) {
        match code {
            Key::Left => self.position += Vector3::new(-1.0, 0.0, 0.0),
            Key::Right => self.position += Vector3::new(1.0, 0.0, 0.0),
            _ => {}
        }
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera<'a> {
    descriptor: CameraDescriptor,
    pub uniform_buffer: wgpu::Buffer,
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

        let camera_uniform_data: [[f32; 4]; 4] = descriptor.get_view_projection_matrix().into();

        queue.write_buffer(
            &uniform_buffer,
            0,
            bytemuck::cast_slice(&camera_uniform_data),
        );

        Self {
            descriptor,
            uniform_buffer,
            bind_group_layout,
            uniform_bind_group,
            queue,
        }
    }

    fn update_camera_buffer(&self) {
        let camera_uniform_data: [[f32; 4]; 4] =
            self.descriptor.get_view_projection_matrix().into();

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&camera_uniform_data),
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.descriptor.aspect = width as f32 / height as f32;
        self.update_camera_buffer();
    }

    pub fn move_camera(&mut self, code: Key) {
        self.descriptor.move_camera(code);
        self.update_camera_buffer();
    }
}
