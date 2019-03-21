use specs::{DispatcherBuilder, Dispatcher, World};
use shrev::{EventChannel};

use crate::event;

pub mod systems;
pub mod viewport;
use crate::app::viewport::Viewport;

pub struct App {
    pub world: World,
    pub update_dispatcher: Dispatcher<'static, 'static>,
}

impl App {
    pub fn new(mut update_dispatcher: Dispatcher<'static, 'static>, ) -> Self {

        let mut world = World::new();
        world.add_resource(Viewport::new(320, 240));
        update_dispatcher.setup(&mut world.res);
        world.maintain();

        App { world, update_dispatcher,  }
    }

    pub fn update(&mut self, delta: u32) {
        self.update_dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }
}