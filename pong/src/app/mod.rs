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

pub struct App<'a, 'b> {
    world: Rc<RefCell<World>>,
    dispatcher: Rc<RefCell<Dispatcher<'a, 'b>>>,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> Self {

//        let mut world = World::new();
//        world.add_resource(EventChannel::<Event>::with_capacity(2000));
//        let mut dispatcher = DispatcherBuilder::new()
//            .with(InputSystem::new(), "input", &[])
//            .build();
//        dispatcher.setup(&mut world.res);
//        world.maintain();

        let world_rc = Rc::new(RefCell::new(World::new()));
        world_rc.borrow_mut().add_resource(EventChannel::<Event>::with_capacity(2000));
        let dispatcher_rc = Rc::new(RefCell::new(
            DispatcherBuilder::new()
                .with(InputSystem::new(), "input", &[])
                .build()));
        dispatcher_rc.borrow_mut().setup(&mut world_rc.borrow_mut().res);
        world_rc.borrow_mut().maintain();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into().unwrap();

        let world_rc2 = world_rc.clone();
        let dispatcher_rc2 = dispatcher_rc.clone();
        let handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut world = world_rc2.borrow_mut();
            world
                .write_resource::<EventChannel<Event>>()
                .single_write(Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: InputEvent::KeyPressed(event.key()),
                    }
                });
            dispatcher_rc2.borrow_mut().dispatch(&world.res);
            world.maintain();
        }) as Box<FnMut(_)>);
        document.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();

        App { world: world_rc, dispatcher: dispatcher_rc }
    }

    pub fn update(&mut self, delta: u32) {
        info!("Updating...");
        self.dispatcher.dispatch(&self.world.res);
    }
}

//fn attach_keyup_callback(document: &Document) -> Result<(), JsValue> {
//    let handler = move |event: web_sys::KeyboardEvent| {
//        info!("Keyup: {}", event.key());
//    };
//
//    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
//    document.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref())?;
//    handler.forget();
//
//    info!("Attaching keyup handler to canvas");
//
//    Ok(())
//}

fn with_int<'a, F>(f: F)
    where
        F: FnOnce(&'a isize),
{
    static X: isize = 3;
    f(&X);
}

fn mai2n() {
    let mut x = None;
    with_int(|y| x = Some(y));
}