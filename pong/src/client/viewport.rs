use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event::{Event, WindowEvent, InputEvent, KeyboardInput};
use crate::input::InputHandler;

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

//-----------------------------


#[derive(Default)]
pub struct ViewportHandler {
    width: u32,
    height: u32,
}

impl ViewportHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn send_event(&mut self,
                      event: &Event,
                      event_handler: &mut EventChannel<WindowEvent>,
    ) {

    }
}


//-----------------------------



pub struct ViewportSystem {
    reader: Option<ReaderId<Event>>,
}

impl ViewportSystem {

    pub fn new() -> Self {
        ViewportSystem {
            reader: None,
        }
    }

    fn process_event(
        event: &Event,
        output: &mut EventChannel<WindowEvent>,
    ) {
        info!("Processing event!")
    }
}

impl<'a> System<'a> for ViewportSystem {
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        Write<'a, EventChannel<WindowEvent>>,
    );

    fn run(&mut self, (input, mut output): Self::SystemData) {
        for event in input.read(&mut self.reader.as_mut().unwrap()) {
            Self::process_event(event, &mut *output);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
        info!("Setting up ViewportSystem");
    }
}