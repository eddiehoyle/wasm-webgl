use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::app::viewport::Viewport;
use crate::event::WindowEvent;

pub struct ViewportSystem {
    reader: Option<ReaderId<WindowEvent>>,
}

impl ViewportSystem {
    pub fn new() -> Self {
        ViewportSystem {
            reader: None,
        }
    }
}

impl<'a> System<'a> for ViewportSystem {
    type SystemData = (
        Read<'a, EventChannel<WindowEvent>>,
        Write<'a, Viewport>,
    );

    fn run(&mut self, (events, mut viewport): Self::SystemData) {
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            match *event {
                WindowEvent::WindowResize(width, height) => {
                    viewport.resize(width, height);
                },
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<WindowEvent>>().register_reader());
        info!("Setting up ViewportSystem");
    }
}