use winit::event_loop::EventLoopWindowTarget;

pub struct WindowCommands<'a> {
    elwt: &'a EventLoopWindowTarget<()>,
}

impl<'a> WindowCommands<'a> {
    pub(super) fn new(elwt: &'a EventLoopWindowTarget<()>) -> Self {
        Self { elwt }
    }

    pub fn exit(&self) {
        self.elwt.exit();
    }
}
