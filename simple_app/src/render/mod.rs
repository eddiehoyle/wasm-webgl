use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

use nalgebra_glm::*;

use crate::shader::manager::ShaderManager;
use crate::shader::ShaderType;
use crate::prim::Rectangle;

pub mod traits;
use crate::render::traits::{Draw};

pub struct WebRenderer {
    shader_manager: ShaderManager,
    rect: Rectangle,
    persp: Mat4x4,
}

impl WebRenderer {
    pub fn new(gl: &GL) -> WebRenderer {
        info!("New WebRenderer");

        let rect = Rectangle::new(&gl, 10.0, 20.0);

        let persp : Mat4x4 = perspective_fov(90.0 ,320.0, 240.0, 0.01, 100.0);

        WebRenderer {shader_manager: ShaderManager::new(&gl), rect, persp}
    }

    pub fn shaders(&self) -> &ShaderManager {
        &self.shader_manager
    }

    pub fn render(&self, gl: &GL) {
        debug!("Rendering...!");
        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.viewport(0, 0, 320, 240);

        self.shader_manager.bind(gl,ShaderType::Persp);
        self.shader_manager.enable(gl);
        self.rect.draw(gl);
        self.shader_manager.disable(gl);
        self.shader_manager.unbind(gl);
    }
}
