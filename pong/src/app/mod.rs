use specs::{DispatcherBuilder, Dispatcher, World};
use shrev::{EventChannel};

use crate::event;

pub struct App {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
}

impl App {
    pub fn new(mut dispatcher: Dispatcher<'static, 'static>) -> Self {

        let mut world = World::new();
        world.add_resource(EventChannel::<event::Event>::new());
        dispatcher.setup(&mut world.res);
        world.maintain();

        App { world, dispatcher }
    }

    pub fn update(&mut self, delta: u32) {
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }
}