mod camera;
mod components;
mod entity;
mod graphics;
mod pass;
mod texture;
mod utils;
mod window;

use camera::{Camera, CameraController, CameraDescriptor};
use components::{Mesh, Position, Scale, Transform};
use entity::Entity;
use graphics::GraphicsContext;
use pass::{Pass, PhongPass};

use texture::Texture;
use window::{Event, Window};

pub async fn run() {
    let window = Window::new();

    let GraphicsContext {
        device,
        queue,
        mut config,
        surface,
    }: GraphicsContext = GraphicsContext::new(&window).await;

    let object = Entity::builder()
        .mesh(
            Mesh::create_cube(&device, &queue, "textures/test.png")
                .await
                .expect("Error when creating cube"),
        )
        .transform(Transform {
            position: Position(1.1, 0.0, -1.9),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    let object2 = Entity::builder()
        .mesh(
            Mesh::create_cube(&device, &queue, "textures/test.png")
                .await
                .expect("Error when creating cube"),
        )
        .transform(Transform {
            position: Position(-1.1, 0.0, -1.9),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    let object3 = Entity::builder()
        .mesh(
            Mesh::create_cube(&device, &queue, "textures/test.png")
                .await
                .expect("Error when creating cube"),
        )
        .transform(Transform {
            position: Position(-1.1, 0.0, -4.1),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    let object4 = Entity::builder()
        .mesh(
            Mesh::create_cube(&device, &queue, "textures/test.png")
                .await
                .expect("Error when creating cube"),
        )
        .transform(Transform {
            position: Position(1.1, 0.0, -4.1),
            scale: Scale(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .build();

    let entities = &vec![object, object2, object3, object4];

    let mut camera = Camera::new(CameraDescriptor {
        position: (0.0, 2.0, 4.0).into(),
        direction: (0.0, 0.0, -1.0).into(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    });

    let camera_controller = CameraController {
        speed: 1.0,
        rotation_speed: 0.01,
    };

    let mut pass = PhongPass::new(&device, &config);

    // Event loop
    window.run(|event, window_commands| match event {
        Event::Resize(width, height) => {
            camera.resize(width, height);
            config.width = width;
            config.height = height;
            surface.configure(&device, &config);
            pass.depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        }
        Event::Draw => {
            pass.draw(&surface, &device, &queue, entities, &camera);
        }
        Event::KeyboardInput(key) => match key {
            window::Key::Left | window::Key::Letter('a') => {
                camera_controller.move_left(&mut camera);
            }
            window::Key::Right | window::Key::Letter('d') => {
                camera_controller.move_right(&mut camera);
            }
            window::Key::Up | window::Key::Letter('w') => {
                camera_controller.move_forward(&mut camera)
            }
            window::Key::Down | window::Key::Letter('s') => {
                camera_controller.move_backwards(&mut camera);
            }
            window::Key::Space => {
                camera_controller.move_up(&mut camera);
            }
            window::Key::ShiftLeft => {
                camera_controller.move_down(&mut camera);
            }
            window::Key::Escape => window_commands.exit(),
            _ => {}
        },
        Event::MouseMove(x, y) => {
            let yaw = x;
            let pitch = -y;
            camera_controller.rotate(&mut camera, (yaw, pitch));
        }
    });
}
