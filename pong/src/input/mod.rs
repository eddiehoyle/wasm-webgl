use shrev::EventChannel;
use crate::event::Event;
use crate::event::{InputEvent, WindowEvent, KeyboardInput};

#[derive(Default)]
pub struct InputHandler {
    is_down: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn send_event(&mut self,
                      event: &Event,
                      event_handler: &mut EventChannel<InputEvent>,
    ) {
        info!("Sending event...");
        match *event {
            Event::WindowEvent { ref event, .. } => match *event {
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        scancode,
                    }
                } => {
                    info!("Keyboard input: {}", scancode);
                }
            }
        }
    }
}
