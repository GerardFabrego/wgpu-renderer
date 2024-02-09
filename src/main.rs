use wgpu_renderer::{App, Entity, ModelDescriptor, Position, Scale, Transform};

fn main() {
    let entity = Entity::builder()
        .model(ModelDescriptor::Cube("textures/test.png"))
        .transform(Transform {
            position: Position(1.1, 0.0, -1.9),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    let entity2 = Entity::builder()
        .model(ModelDescriptor::File("cube.obj"))
        .transform(Transform {
            position: Position(-2.0, 0.5, -1.5),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    App::new().add_entity(&entity).add_entity(&entity2).run();
}
