use crate::components::{Mesh, Transform};

pub struct Entity {
    pub mesh: Mesh,
    transform: Transform,
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
    }
}

pub struct EntityBuilder {
    mesh: Option<Mesh>,
    transform: Transform,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            mesh: None,
            transform: Transform::default(),
        }
    }

    pub fn mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(self) -> Entity {
        Entity {
            mesh: self.mesh.expect("Missing mesh when creating entity"),
            transform: self.transform,
        }
    }
}
