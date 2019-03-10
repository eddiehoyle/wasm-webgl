use shrev::EventChannel;
use crate::event::Event;
use crate::event::{InputEvent, WindowEvent, KeyboardInput};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct InputHandler {
    keyset: HashSet<String>,
}

impl InputHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn send_event(&mut self,
                      event: &Event,
                      event_handler: &mut EventChannel<InputEvent>,
    ) {
        match *event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::KeyboardInput { ref input, .. } => match input {
                    InputEvent::KeyPressed(key) => {
                        if let None = self.keyset.get(key) {
                            info!("Pressed: {}", &key);
                            self.keyset.insert(key.clone());
                        }
                    },
                    InputEvent::KeyReleased(key) => {
                        if let Some(key) = self.keyset.get(key) {
                            info!("Released: {}", &key);
                        }
                        self.keyset.remove(key);

                    }
                }
            }
        }
    }
}