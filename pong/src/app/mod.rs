use specs::{DispatcherBuilder, Dispatcher, World};
use shrev::{EventChannel};
use crate::event::Event;
use crate::event::system::InputSystem;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::rc::Rc;
use std::cell::RefCell;
use crate::event::*;

pub struct App {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
}

impl App {
    pub fn new(mut dispatcher: Dispatcher<'static, 'static>) -> Self {

        let mut world = World::new();
        world.add_resource(EventChannel::<Event>::new());
//        let mut dispatcher = DispatcherBuilder::new()
//            .with(InputSystem::new(), "input", &[])
//            .build();
        dispatcher.setup(&mut world.res);
        world.maintain();

        App { world, dispatcher }
    }

    pub fn update(&mut self, delta: u32) {
        self.dispatcher.dispatch(&self.world.res);
    }
}