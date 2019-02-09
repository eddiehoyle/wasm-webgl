use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;

pub struct App {
}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn msg(&self, msg: Msg) {
        match msg {
            Msg::Tick(delta) => {
//                console::log_1(&JsValue::from(format!("Msg::Tick({})", delta)))
            }
        }
    }
}

pub enum Msg {
    Tick(f32),
}