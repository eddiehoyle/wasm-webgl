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
use crate::prim::Rectangle;

pub mod traits;
use crate::render::traits::{Draw};

pub struct WebRenderer {
    shader_manager: ShaderManager,
    rect: Rectangle,
}

impl WebRenderer {
    pub fn new(gl: &GL) -> WebRenderer {
        info!("New WebRenderer");

        let indices = [
            0, 1, 3,
            3, 1, 2,
        ];
        let vertices = [
            -0.5, 0.5, 0.0,
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.5, 0.5, 0.0,
        ];

        let rect = Rectangle::new(&gl, &indices, &vertices);

        WebRenderer {shader_manager: ShaderManager::new(&gl), rect}
    }

    pub fn shaders(&self) -> &ShaderManager {
        &self.shader_manager
    }

    pub fn render(&self, gl: &GL) {
        debug!("Rendering...!");
        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.viewport(0, 0, 320, 240);

        self.shader_manager.bind(gl,ShaderType::Simple);
        gl.enable_vertex_attrib_array(0);
        self.rect.draw(gl);
        gl.disable_vertex_attrib_array(0);
        self.shader_manager.unbind(gl);
    }
}
