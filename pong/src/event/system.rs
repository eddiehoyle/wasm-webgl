use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event;

pub struct EventSystem {
    reader: Option<ReaderId<event::Event>>,
}

impl EventSystem {

    pub fn new() -> Self {
        EventSystem {
            reader: None,
        }
    }

    fn process_event(event: &event::Event, output: &mut EventChannel<event::WindowEvent>) {
        match *event {
            event::Event::WindowEvent { ref event, .. } => match event {
                event::WindowEvent::WindowResize(width, height) => {},
            }
        }
    }
}

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        Read<'a, EventChannel<event::Event>>,
        Write<'a, EventChannel<event::WindowEvent>>,
    );

    fn run(&mut self, (input, mut output): Self::SystemData) {
        for event in input.read(&mut self.reader.as_mut().unwrap()) {
            Self::process_event(event, &mut *output);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<event::Event>>().register_reader());
        info!("Setting up WindowSystem");
    }
}