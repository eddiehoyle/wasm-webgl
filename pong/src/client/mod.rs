use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use specs::World;
use specs::DispatcherBuilder;
use shrev::EventChannel;
use specs::Dispatcher;
use crate::input::system::InputSystem;
use crate::client::dom::create_webgl_context;
use crate::app::App;

use std::rc::Rc;
use std::cell::RefCell;
use crate::event::{Event, WindowEvent, InputEvent};
use crate::render::WebRenderer;
use crate::render::system::RenderSystem;

mod dom;
mod viewport;

#[wasm_bindgen]
pub struct WebClient {
    app: Rc<RefCell<App>>,
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

        let gl_rc = Rc::new(create_webgl_context().unwrap());

        let dispatcher = DispatcherBuilder::new()
            .with(InputSystem::new(), "input", &[])
            .with_barrier()
//            .with(RenderSystem::new(), "render", &[])
            .build();

        let app_rc = Rc::new(RefCell::new(App::new(dispatcher)));
        WebClient { app: app_rc }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        info!("WebClient starting...");

        attach_keydown_event(self.app.clone());
        attach_keyup_event(self.app.clone());

        Ok(())
    }

    pub fn update(&mut self, delta: u32) {
        self.app.borrow_mut().update(delta);
    }
}

pub fn attach_keydown_event(app: Rc<RefCell<App>>) {
    info!("Attaching keydown");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into().unwrap();
    let app = app.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        app.borrow_mut().world
            .write_resource::<EventChannel<Event>>()
            .single_write(Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: InputEvent::KeyPressed(event.key()),
                }
            });
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn attach_keyup_event(app: Rc<RefCell<App>>) {
    info!("Attaching keyup");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into().unwrap();
    let app = app.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        app.borrow_mut().world
            .write_resource::<EventChannel<Event>>()
            .single_write(Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: InputEvent::KeyReleased(event.key()),
                }
            });
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}