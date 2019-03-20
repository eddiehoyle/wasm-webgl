use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use specs::World;
use specs::DispatcherBuilder;
use shrev::EventChannel;
use specs::Dispatcher;
use crate::input::system::InputSystem;
use crate::app::App;

use std::rc::Rc;
use std::cell::RefCell;
use crate::event;
use crate::render::WebRenderer;
use crate::render::system::RenderSystem;
use crate::event::system::*;
use crate::client::dom::*;
use crate::app::viewport::Viewport;

pub(crate) mod dom;

#[wasm_bindgen]
pub struct WebClient {
    app: Rc<RefCell<App>>,
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

        let canvas = init_canvas().unwrap();

        let update_dispatcher = DispatcherBuilder::new()
            .with(EventSystem::new(), "window", &[])
            .with(InputSystem::new(), "input", &[])
            .with_thread_local(RenderSystem::new(canvas))
            .build();

        let app_rc = Rc::new(RefCell::new(App::new(update_dispatcher)));

        WebClient { app: app_rc }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        info!("WebClient starting...");

        attach_keydown_event(self.app.clone());
        attach_keyup_event(self.app.clone());
        attach_viewport_resize(self.app.clone());

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
            .write_resource::<EventChannel<event::InputEvent>>()
            .single_write(event::InputEvent::KeyPressed(event.key()));
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
            .write_resource::<EventChannel<event::InputEvent>>()
            .single_write(event::InputEvent::KeyReleased(event.key()));
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn attach_viewport_resize(app: Rc<RefCell<App>>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into().unwrap();
    let app = app.clone();
    let closure = Closure::wrap(Box::new(move || {
        let size : (u32, u32) = app.borrow().world.read_resource::<Viewport>().size();
        if canvas.width() != size.0 || canvas.height() != size.1 {
            app.borrow_mut().world
                .write_resource::<EventChannel<event::WindowEvent>>()
                .single_write(event::WindowEvent::WindowResize(size.0, size.1));
            info!("Resized: {}, {}", canvas.width(), canvas.height());
        }

    }) as Box<FnMut()>);
    window.set_interval_with_callback_and_timeout_and_arguments(
        closure.as_ref().unchecked_ref(),
        250,
        &js_sys::Array::new()
    ).unwrap();
    closure.forget();
}