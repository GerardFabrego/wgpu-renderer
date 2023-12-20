mod camera;
mod graphics;
mod pass;
mod primitives;
mod texture;
mod utils;
mod vertex;
mod window;

use camera::{Camera, CameraDescriptor};
use cgmath::Vector3;
use graphics::GraphicsContext;
use pass::{Pass, PhongPass};
use primitives::Cube;
use window::{Event, Window};

pub async fn run() {
    let window = Window::new();

    let GraphicsContext {
        device,
        queue,
        mut config,
        surface,
    }: GraphicsContext = GraphicsContext::new(&window).await;

    let object = Cube::new(&device, &queue)
        .await
        .expect("Error when creating cube");

    let camera_descriptor = CameraDescriptor {
        position: (0.0, 2.0, 4.0).into(),
        direction: (0.0, 0.0, -1.0).into(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera = Camera::new(camera_descriptor);

    let mut pass = PhongPass::new(&device, &config);
    pass.update_camera_buffer(&queue, &camera);

    // Event loop
    window.run(|event, window_commands| match event {
        Event::Resize(width, height) => {
            camera.resize(width, height);
            config.width = width;
            config.height = height;
            surface.configure(&device, &config);
            pass.update_camera_buffer(&queue, &camera)
        }
        Event::Draw => {
            pass.draw(&surface, &device, &queue, &object);
        }
        Event::KeyboardInput(key) => match key {
            window::Key::Left | window::Key::Letter('a') => {
                camera.translate(Vector3::new(-1.0, 0.0, 0.0));
                pass.update_camera_buffer(&queue, &camera);
            }
            window::Key::Right | window::Key::Letter('d') => {
                camera.translate(Vector3::new(1.0, 0.0, 0.0));
                pass.update_camera_buffer(&queue, &camera);
            }
            window::Key::Up | window::Key::Letter('w') => {
                camera.translate(Vector3::new(0.0, 1.0, 0.0));
                pass.update_camera_buffer(&queue, &camera);
            }
            window::Key::Down | window::Key::Letter('s') => {
                camera.translate(Vector3::new(0.0, -1.0, 0.0));
                pass.update_camera_buffer(&queue, &camera);
            }
            window::Key::Escape => window_commands.exit(),
            _ => {}
        },
        Event::MouseMove(y, x) => {
            camera.rotate(y, x);
            pass.update_camera_buffer(&queue, &camera);
        }
    });
}
