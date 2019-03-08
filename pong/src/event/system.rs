use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event::{Event, WindowEvent, InputEvent, KeyboardInput};
use crate::input::InputHandler;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

pub struct InputSystem {
    reader: Option<ReaderId<Event>>,
}

impl InputSystem {

    pub fn new() -> Self {
        InputSystem {
            reader: None,
        }
    }

    fn process_event(
        event: &Event,
        handler: &mut InputHandler,
        output: &mut EventChannel<InputEvent>,
    ) {
        info!("Processing event...");
        handler.send_event(event, output);
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        Write<'a, InputHandler>,
        Write<'a, EventChannel<InputEvent>>,
    );

    fn run(&mut self, (input, mut handler, mut output): Self::SystemData) {
        for event in input.read(&mut self.reader.as_mut().unwrap()) {
            Self::process_event(event, &mut *handler, &mut *output);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
        info!("Setting up InputSystem");
    }
}