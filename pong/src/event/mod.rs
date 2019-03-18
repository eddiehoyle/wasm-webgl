pub mod system;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    WindowEvent {
        event: WindowEvent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    WindowResize(u32, u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    KeyPressed(String),
    KeyReleased(String),
}
