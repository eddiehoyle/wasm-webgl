use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;

use std::option;
use crate::shader::Shader;

pub struct ShaderManager {
    active: Option<Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        console::log_1(&JsValue::from("ShaderManager::new()"));
        ShaderManager{ active: None }
    }
}

