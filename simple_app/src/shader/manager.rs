use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::cmp::Ordering;

use std::option;
use crate::shader::{Shader, ShaderType};
use std::collections::HashMap;

pub struct ShaderManager {
    active: Option<Shader>,
    shaders: Vec<Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        console::log_1(&JsValue::from("ShaderManager::new()"));
        ShaderManager{ active: None, shaders: Vec::new() }
    }

    pub fn bind(&self, gl: &GL, type_: &ShaderType) {
        if let Ok(shader) = self.shaders.binary_search_by_key(type_,
            |s| {s.type_()}) {
            gl.use_program(Some(&shader.program()))
        }
    }

    pub fn unbind(&self, context: &GL) {
//        context.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
    }
}

