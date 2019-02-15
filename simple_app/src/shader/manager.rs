use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::cmp::Ordering;

use std::option;
use crate::shader::*;
use std::collections::HashMap;
use std::cell::Cell;
use core::borrow::Borrow;

type ShaderMap = HashMap<ShaderType, Shader>;

static SIMLPE_VS: &'static str = include_str!("./../../dist/static-vertex.glsl");
static SIMPLE_FS: &'static str = include_str!("./../../dist/static-fragment.glsl");

pub struct ShaderManager {
    active: RefCell<Option<ShaderType>>,
    shaders: Vec<Shader>,
}

impl ShaderManager {
    pub fn new(gl: &GL) -> Self {
        info!("New ShaderManager");
        let mut shaders = Vec::new();

        let shader = Shader::new(gl.borrow(),
                                 SIMLPE_VS,
                                 SIMPLE_FS,
                                 &["position"],
                                 &[],
                                 ShaderType::Simple);
        match shader {
            Ok(shader) => shaders.push(shader),
            Err(e) => error!("ERROR compiling '{:?}' shader!\n{:?}", ShaderType::Simple, e),
        }

        shaders[0].attributes();

        ShaderManager{ active: RefCell::new(None), shaders }
    }

    pub fn bind(&self, gl: &GL, type_: ShaderType) {
        if let Some(shader) = self.shaders.iter().find(
            |shader|{ shader.type_() == type_ }) {
            debug!("Binding shader: {:?}", type_);
            gl.use_program(Some(shader.program()));
            *self.active.borrow_mut() = Some(shader.type_().clone())
        }
    }

    pub fn unbind(&self, gl: &GL) {
        if let Some(type_) = *self.active.borrow() {
            debug!("Uninding shader: {:?}", type_);
            gl.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
        }
    }
}

