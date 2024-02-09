#[derive(Clone, Debug)]
pub struct Position(pub f32, pub f32, pub f32);

impl Default for Position {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl From<&Position> for cgmath::Vector3<f32> {
    fn from(position: &Position) -> cgmath::Vector3<f32> {
        let &Position(x, y, z) = position;
        cgmath::Vector3::new(x, y, z)
    }
}

#[derive(Clone)]

pub struct Scale(pub f32, pub f32, pub f32);

impl Default for Scale {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl From<&Scale> for cgmath::Vector3<f32> {
    fn from(scale: &Scale) -> cgmath::Vector3<f32> {
        let &Scale(x, y, z) = scale;
        cgmath::Vector3::new(x, y, z)
    }
}

#[derive(Clone)]
pub struct Rotation(pub f32, pub f32, pub f32, pub f32);

impl Default for Rotation {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<&Rotation> for cgmath::Quaternion<f32> {
    fn from(rotation: &Rotation) -> cgmath::Quaternion<f32> {
        let &Rotation(x, y, z, w) = rotation;
        cgmath::Quaternion::new(x, y, z, w)
    }
}

#[derive(Default, Clone)]
pub struct Transform {
    pub position: Position,
    pub scale: Scale,
    pub rotation: Rotation,
}

pub type TransformRaw = [[f32; 4]; 4];

impl From<&Transform> for TransformRaw {
    fn from(transform: &Transform) -> Self {
        let translation_matrix =
            cgmath::Matrix4::from_translation(cgmath::Vector3::from(&transform.position));

        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(
            transform.scale.0,
            transform.scale.1,
            transform.scale.2,
        );

        let rotation_matrix = cgmath::Matrix4::from(cgmath::Quaternion::from(&transform.rotation));

        (translation_matrix * scale_matrix * rotation_matrix).into()
    }
}
