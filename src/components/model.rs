use super::{geometry::Material, Mesh};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}
