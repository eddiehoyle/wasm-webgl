pub mod system;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    WindowEvent {
        event: WindowEvent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    KeyboardInput {
        input: InputEvent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    KeyPressed(String),
    KeyReleased(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardInput {
    pub scancode: u32,
}