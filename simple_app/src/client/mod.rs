use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

mod dom;
mod controls;

use crate::texture;
use crate::render::WebRenderer;
use crate::app::{App, Msg};
use crate::prim::rect::Rectangle;
use crate::client::controls::*;

#[wasm_bindgen]
pub struct WebClient {
    gl: Rc<GL>,
    app: App,
    render: WebRenderer,
    clock: RefCell<f32>,
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        info!("New WebClient");

        let gl = Rc::new(dom::create_webgl_context().unwrap());
        let app = App::new();
        let render = WebRenderer::new(&gl);

//        append_controls(&app).expect("Append controls");

        WebClient { gl, app, render, clock: RefCell::new(0.0) }
    }

    pub fn start(&self) -> Result<(), JsValue> {
        info!("WebClient starting...");
        texture::Texture::new( &self.gl, "cat.png", 0);
        Ok(())
    }

    pub fn update(&self, delta: u32) {
        self.app.msg(Msg::Tick(delta as f32 / 1000.));
        let clock = *self.clock.borrow() + (delta as f32 / 1000.0);
        *self.clock.borrow_mut() = clock;
    }

    pub fn render(&mut self) {
        self.render.render(&self.gl, &self.clock.borrow());
//        info!("Clock: {}", self.clock.borrow());
//        self.render.render(&self.gl, &1.0);

    }
}