use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event;
use crate::input::InputHandler;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

pub struct InputSystem {
    reader: Option<ReaderId<event::InputEvent>>,
}

impl InputSystem {

    pub fn new() -> Self {
        InputSystem {
            reader: None,
        }
    }

    fn process_event(
        event: &event::InputEvent,
        handler: &mut InputHandler,
    ) {
        match event {
            event::InputEvent::KeyPressed(key) => {
                if !handler.is_pressed(key) {
                    info!("Key press: {}", key);
                }
                handler.press(key);
            },
            event::InputEvent::KeyReleased(key) => {
                handler.release(key);
                info!("Key release: {}", key);
            },
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, EventChannel<event::InputEvent>>,
        Write<'a, InputHandler>,
    );

    fn run(&mut self, (input, mut handler): Self::SystemData) {
        for event in input.read(&mut self.reader.as_mut().unwrap()) {
            Self::process_event(event, &mut *handler);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<event::InputEvent>>().register_reader());
        info!("Setting up InputSystem");
    }
}