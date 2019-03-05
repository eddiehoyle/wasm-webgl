#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    WindowEvent {
        event: WindowEvent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    KeyboardInput {
        input: KeyboardInput,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    KeyPressed(u32),
    KeyReleased(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardInput {
    pub scancode: u32,
}