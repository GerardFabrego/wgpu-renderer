pub struct Transform {
    pub position: cgmath::Vector3<f32>,
    pub scale: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    m_matrix: [[f32; 4]; 4],
}

impl TransformRaw {
    pub fn from(transform: &Transform) -> Self {
        Self {
            m_matrix: (cgmath::Matrix4::from_translation(transform.position)
                * cgmath::Matrix4::from_nonuniform_scale(
                    transform.scale.x,
                    transform.scale.y,
                    transform.scale.z,
                )
                * cgmath::Matrix4::from(transform.rotation))
            .into(),
        }
    }
}
