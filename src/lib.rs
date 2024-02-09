mod camera;
mod components;
mod entity;
mod graphics;
mod pass;
mod texture;
mod utils;
mod window;

use camera::{Camera, CameraController, CameraDescriptor};
use cgmath::Deg;
use components::Model;
pub use components::{ModelDescriptor, Position, Scale, Transform};
use entity::BakedEntity;
pub use entity::Entity;
use graphics::GraphicsContext;
use pass::{Pass, PhongPass};

use texture::Texture;
use utils::load_model;
use window::{Event, Window};

pub struct App<'a> {
    window: Window,
    graphics_context: GraphicsContext<'a>,
    world: Vec<BakedEntity>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let window = Window::new();

        let graphics_context = pollster::block_on(GraphicsContext::new(&window));

        App {
            window,
            graphics_context,
            world: vec![],
        }
    }

    pub fn add_entity(mut self, entity: &'a Entity) -> Self {
        let baked_entity = self.bake(entity);
        self.world.push(baked_entity);
        self
    }

    fn bake(&self, entity: &Entity) -> BakedEntity {
        let model = match entity.model_descriptor {
            ModelDescriptor::Cube(texture) => Model::cube(
                &self.graphics_context.device,
                &self.graphics_context.queue,
                texture,
            ),
            ModelDescriptor::File(path) => load_model(
                path,
                &self.graphics_context.device,
                &self.graphics_context.queue,
            ),
        }
        .unwrap();

        BakedEntity {
            model,
            transform: entity.transform.clone(),
        }
    }

    pub fn run(self) {
        let GraphicsContext {
            surface,
            mut config,
            device,
            queue,
        } = self.graphics_context;

        let mut camera = Camera::new(
            (0.0, 2.0, 4.0),
            Deg(-90.0),
            Deg(0.0),
            CameraDescriptor {
                aspect: config.width as f32 / config.height as f32,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
        );

        let camera_controller = CameraController {
            speed: 1.0,
            rotation_speed: 0.01,
        };

        let mut pass = PhongPass::new(&device, &config);

        // Event loop
        self.window.run(|event, window_commands| match event {
            Event::Resize(width, height) => {
                camera.resize(width, height);
                config.width = width;
                config.height = height;
                surface.configure(&device, &config);
                pass.depth_texture =
                    Texture::create_depth_texture(&device, &config, "depth_texture");
            }
            Event::Draw => {
                pass.draw(&surface, &device, &queue, &self.world, &camera);
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
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}
