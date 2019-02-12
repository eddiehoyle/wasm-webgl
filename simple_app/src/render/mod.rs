use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

use crate::shader;

pub struct WebRenderer {
}

impl WebRenderer {
    pub fn new(gl: Rc<GL>) -> WebRenderer {
        console::log_1(&JsValue::from("new WebRenderer"));

        let m = shader::manager::ShaderManager::new();

        WebRenderer {}
    }
}
