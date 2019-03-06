use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use specs::World;
use specs::DispatcherBuilder;
use shrev::EventChannel;
use specs::Dispatcher;
use crate::event::Event;
use crate::event::system::InputSystem;
use crate::client::dom::create_webgl_context;
use crate::app::App;

mod dom;

#[wasm_bindgen]
pub struct WebClient {
    app: App<'static, 'static>,
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        info!("New WebClient!");

        create_webgl_context().unwrap();

        WebClient { app: App::new() }
    }

    pub fn start(&self) -> Result<(), JsValue> {
        info!("WebClient starting...");
        Ok(())
    }

    pub fn update(&mut self, delta: u32) {
        self.app.update(delta);
    }

    pub fn render(&mut self) {
    }
}