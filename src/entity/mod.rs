use crate::components::{Model, ModelDescriptor, Transform};

pub struct BakedEntity {
    pub model: Model,
    pub transform: Transform,
}
pub struct Entity<'a> {
    pub model_descriptor: ModelDescriptor<'a>,
    pub transform: Transform,
}

impl Entity<'_> {
    pub fn builder() -> EntityBuilder<'static> {
        EntityBuilder::new()
    }
}

pub struct EntityBuilder<'a> {
    model_descriptor: Option<ModelDescriptor<'a>>,
    transform: Transform,
}

impl<'a> EntityBuilder<'a> {
    pub fn new() -> Self {
        Self {
            model_descriptor: None,
            transform: Transform::default(),
        }
    }

    pub fn model(mut self, model_descriptor: ModelDescriptor<'a>) -> Self {
        self.model_descriptor = Some(model_descriptor);
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(self) -> Entity<'a> {
        Entity {
            model_descriptor: self
                .model_descriptor
                .expect("Missing model when creating entity"),
            transform: self.transform,
        }
    }
}
