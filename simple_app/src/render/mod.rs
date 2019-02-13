use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

use crate::shader::manager::ShaderManager;
use crate::shader::ShaderType;


pub struct WebRenderer {
    shader_manager: ShaderManager,
}

impl WebRenderer {
    pub fn new(gl: &GL) -> WebRenderer {
        info!("New WebRenderer");
        WebRenderer {shader_manager: ShaderManager::new(&gl)}
    }

    pub fn shaders(&self) -> &ShaderManager {
        &self.shader_manager
    }

    pub fn render(&self, gl: &GL) {
        info!("Rendering...!");
        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.viewport(0, 0, 320, 240);
    }
}
