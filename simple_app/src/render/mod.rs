use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

use nalgebra_glm as glm;

use crate::shader::manager::ShaderManager;
use crate::shader::ShaderType;
use crate::prim::{Rectangle, camera::Camera};

pub mod traits;
use crate::render::traits::{Draw};

pub struct WebRenderer {
    shader_manager: ShaderManager,
    rect: Rectangle,
    camera: Camera,
}

impl WebRenderer {
    pub fn new(gl: &GL) -> WebRenderer {
        info!("New WebRenderer");

        let rect = Rectangle::new(&gl, 10.0, 20.0);
        let camera = Camera::new();

        WebRenderer {shader_manager: ShaderManager::new(&gl), rect, camera}
    }

    pub fn shaders(&self) -> &ShaderManager {
        &self.shader_manager
    }

    pub fn render(&self, gl: &GL, clock: &f32) {
        let clock = clock * 2.0;
        debug!("Rendering...!");
        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.viewport(0, 0, 320, 240);

        self.shader_manager.bind(gl,ShaderType::Persp);
        self.shader_manager.enable(gl);
        self.shader_manager.load_mat4(gl, "uProjection", self.camera.projection());
        self.shader_manager.load_mat4(gl, "uView",  &glm::translate(&glm::identity(), &glm::Vec3::new(0.0, 0.0, -20.0)));

//        let model_matrix = glm::translate(&glm::identity(), &glm::Vec3::new(clock.sin() * 30.0, clock.cos() * -10.0, 0.0));
        let model_matrix = glm::rotate( &glm::identity(), clock.sin(), &glm::Vec3::new(0.0, 1.0, 0.0));
        self.shader_manager.load_mat4(gl, "uModel", &model_matrix);

        self.rect.draw(gl);
        self.shader_manager.disable(gl);
        self.shader_manager.unbind(gl);
    }
}
