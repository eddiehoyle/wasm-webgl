use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::cmp::Ordering;

use std::option;
use std::collections::HashMap;
use std::cell::Cell;
use core::borrow::Borrow;
use crate::render::shader::{ShaderType, Shader};

type ShaderMap = HashMap<ShaderType, Shader>;

//static STATIC_VS: &'static str = include_str!("./../../dist/static-vertex.glsl");
//static STATIC_FS: &'static str = include_str!("./../../dist/static-fragment.glsl");

//static PERSP_VS: &'static str = include_str!("./../../dist/persp-vertex.glsl");
//static PERSP_FS: &'static str = include_str!("./../../dist/persp-fragment.glsl");


static STATIC_VS: &'static str = "";
static STATIC_FS: &'static str = "";

static PERSP_VS: &'static str = "";
static PERSP_FS: &'static str = "";

pub struct ShaderManager {
    active: RefCell<Option<ShaderType>>,
    shaders: Vec<Shader>,
}

impl ShaderManager {
    pub fn new(gl: &GL) -> Self {
        info!("New ShaderManager");
        let mut shaders = Vec::new();

        let shader = Shader::new(gl.borrow(),
                                 STATIC_VS,
                                 STATIC_FS,
                                 &["position"],
                                 &[],
                                 ShaderType::Simple);
        match shader {
            Ok(shader) => shaders.push(shader),
            Err(e) => error!("ERROR compiling '{:?}' shader!\n{:?}", ShaderType::Simple, e),
        }

        let shader = Shader::new(gl.borrow(),
                                 PERSP_VS,
                                 PERSP_FS,
                                 &["aPosition"],
                                 &["uProjection", "uModel", "uView"],
                                 ShaderType::Persp);
        match shader {
            Ok(shader) => shaders.push(shader),
            Err(e) => error!("ERROR compiling '{:?}' shader!\n{:?}", ShaderType::Persp, e),
        }

        ShaderManager{ active: RefCell::new(None), shaders }
    }

//    pub fn load_mat4(&self, gl: &GL, name: &str, matrix: &glm::Mat4) {
//        if let Some(type_) = *self.active.borrow() {
//            let shader = self.shaders.iter().find(
//                |shader| { shader.type_() == type_ }).unwrap();
//            match shader.uniforms().get(name) {
//                Some(location) => {
//                    let mut array = [0.0; 16];
//                    array.copy_from_slice(matrix.as_slice());
//                    gl.uniform_matrix4fv_with_f32_array(Some(&location), false, &mut array);
//                },
//                None => {
//                    error!("Uniform '{}' not found for shader: {:?}", name, type_);
//                }
//            }
//        }
//    }

    pub fn bind(&self, gl: &GL, type_: ShaderType) {
        if let Some(shader) = self.shaders.iter().find(
            |shader|{ shader.type_() == type_ }) {
//            debug!("Binding shader: {:?}", type_);
            gl.use_program(Some(shader.program()));
            *self.active.borrow_mut() = Some(shader.type_().clone())
        }
    }

    pub fn enable(&self, gl: &GL) {
        if let Some(type_) = *self.active.borrow() {
            let shader = self.shaders.iter().find(
                |shader|{ shader.type_() == type_ }).unwrap();
            for attr in shader.attributes().values() {
//                debug!("Enabling vertex attrib: {}", attr);
                gl.enable_vertex_attrib_array(*attr as u32);
            }
        }
    }

    pub fn unbind(&self, gl: &GL) {
        if let Some(type_) = *self.active.borrow() {
//            debug!("Unbinding shader: {:?}", type_);
            gl.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
        }
    }

    pub fn disable(&self, gl: &GL) {
        if let Some(type_) = *self.active.borrow() {
            let shader = self.shaders.iter().find(
                |shader|{ shader.type_() == type_ }).unwrap();
            for attr in shader.attributes().values() {
//                debug!("Disabling vertex attrib: {}", attr);
                gl.disable_vertex_attrib_array(*attr as u32);
            }
        }
    }

    pub fn active(&self) -> Option<&Shader> {
        if let Some(type_) = *self.active.borrow() {
            return self.shaders.iter().find(
                |shader|{ shader.type_() == type_ })
        }
        None
    }
}

