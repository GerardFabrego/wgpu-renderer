use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{self, WindowBuilder},
};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_title("WGPU Renderer")
            .build(&event_loop)
            .expect("There was an error when building the window");
        Self { event_loop, window }
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.height, size.width)
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn run(&self, callback: impl FnMut(Event<()>, &EventLoopWindowTarget<()>)) {
        self.event_loop.run(callback);
    }
}
