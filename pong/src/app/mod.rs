use specs::{DispatcherBuilder, Dispatcher, World};
use shrev::{EventChannel};
use crate::event::Event;
use crate::event::system::InputSystem;

pub struct App<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> Self {

        let mut world = World::new();
        world.add_resource(EventChannel::<Event>::with_capacity(2000));
        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem::new(), "input", &[])
            .build();
        dispatcher.setup(&mut world.res);
        world.maintain();

        App{ world, dispatcher }
    }

    pub fn update(&mut self, delta: u32) {
        info!("Updating...");
        self.dispatcher.dispatch(&self.world.res);
    }
}