mod commands;
mod events;

use std::sync::Arc;

use winit::{
    event::{DeviceEvent, Event as WinitEvent, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode as WinitKeyCode, PhysicalKey},
    window::{self, WindowBuilder},
};

pub use self::{
    commands::WindowCommands,
    events::{Event, Key},
};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: Arc<window::Window>,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_title("WGPU Renderer")
            .build(&event_loop)
            .expect("There was an error when building the window");
        Self {
            event_loop,
            window: Arc::new(window),
        }
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.height, size.width)
    }

    pub fn run(self, mut callback: impl FnMut(Event, WindowCommands)) {
        self.event_loop
            .run(move |event, elwt| {
                let event = match event {
                    WinitEvent::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                            None
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: PhysicalKey::Code(code),
                                    ..
                                },
                            ..
                        } => {
                            let key = match code {
                                WinitKeyCode::ArrowLeft => Key::Left,
                                WinitKeyCode::ArrowRight => Key::Right,
                                WinitKeyCode::ArrowUp => Key::Up,
                                WinitKeyCode::ArrowDown => Key::Down,
                                WinitKeyCode::Space => Key::Space,
                                WinitKeyCode::Escape => Key::Escape,
                                WinitKeyCode::ShiftLeft => Key::ShiftLeft,
                                code if code >= WinitKeyCode::Digit0
                                    && code <= WinitKeyCode::Digit9 =>
                                {
                                    Key::Digit(code as u8 - WinitKeyCode::Digit0 as u8)
                                }
                                code if code >= WinitKeyCode::KeyA
                                    && code <= WinitKeyCode::KeyZ =>
                                {
                                    Key::Letter(
                                        ((code as u8 - WinitKeyCode::KeyA as u8) + 97_u8) as char,
                                    )
                                }
                                _ => Key::Other,
                            };
                            Some(Event::KeyboardInput(key))
                        }
                        WindowEvent::Resized(new_size) => {
                            Some(Event::Resize(new_size.width, new_size.height))
                        }
                        WindowEvent::RedrawRequested => Some(Event::Draw),
                        _ => None,
                    },
                    WinitEvent::DeviceEvent {
                        event: DeviceEvent::MouseMotion { delta },
                        ..
                    } => Some(Event::MouseMove(delta.0 as f32, delta.1 as f32)),

                    WinitEvent::AboutToWait => {
                        self.window.request_redraw();
                        None
                    }
                    _ => None,
                };

                if let Some(event) = event {
                    let window_actions = WindowCommands::new(elwt);
                    callback(event, window_actions);
                }
            })
            .expect("There was an error while running the event loop");
    }
}
