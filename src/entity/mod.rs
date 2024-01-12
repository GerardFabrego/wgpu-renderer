use crate::components::{Mesh, Model, Transform};

pub struct Entity {
    pub model: Model,
    pub transform: Transform,
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
    }
}

pub struct EntityBuilder {
    model: Option<Model>,
    transform: Transform,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            transform: Transform::default(),
        }
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(self) -> Entity {
        Entity {
            model: self.model.expect("Missing model when creating entity"),
            transform: self.transform,
        }
    }
}
