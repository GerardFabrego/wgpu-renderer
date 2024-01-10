pub enum Event {
    Resize(u32, u32),
    KeyboardInput(Key),
    MouseMove(f32, f32),
    Draw,
}

#[derive(Debug)]
pub enum Key {
    Digit(u8),
    Letter(char),
    Escape,
    Up,
    Down,
    Left,
    Right,
    Space,
    Control,
    Other,
}
