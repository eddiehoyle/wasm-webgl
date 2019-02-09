use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

mod dom;

use crate::texture;
use crate::render::WebRenderer;
use crate::app::{App, Msg};

#[wasm_bindgen]
pub struct WebClient {
    gl: Rc<GL>,
    app: App,
    render: WebRenderer,
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console::log_1(&JsValue::from("New WebClient"));
        let gl = Rc::new(dom::create_webgl_context().unwrap());
        let app = App::new();
        let render = WebRenderer::new(gl.clone());
        WebClient { gl, app, render }
    }

    pub fn member(&self) {
        console::log_1(&JsValue::from("member"));
    }

    pub fn start(&self) -> Result<(), JsValue> {
        console::log_1(&JsValue::from("Starting web client..."));
        texture::Texture::new( &self.gl, "cat.png", 0);
        Ok(())
    }

    pub fn update(&self, delta: u32) {
        self.app.msg(Msg::Tick(delta as f32 / 1000.));
    }

    pub fn render(&mut self) {

    }
}