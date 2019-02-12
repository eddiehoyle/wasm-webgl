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

pub struct ShaderManager<'a> {
    active: Cell<Option<&'a Shader>>,
//    shaders: Vec<&'a Shader>,
    shaders: ShaderMap,
}

type ShaderMap = HashMap<ShaderType, Shader>;

static SIMLPE_VS: &'static str = include_str!("./../../dist/static-vertex.glsl");
static SIMPLE_FS: &'static str = include_str!("./../../dist/static-fragment.glsl");

impl<'a> ShaderManager<'a> {
    pub fn new(gl: &GL) -> Self {
        console::log_1(&JsValue::from("ShaderManager::new()"));

        let simple = Shader::new(gl.borrow(),
                                    SIMLPE_VS,
                                    SIMPLE_FS,
                                    &[],
                                    &[],
                                    ShaderType::Simple);

        let mut shaders = HashMap::new();
        shaders.insert(ShaderType::Simple, Shader::new(gl.borrow(),
                                                       SIMLPE_VS,
                                                       SIMPLE_FS,
                                                       &[],
                                                       &[], ShaderType::Simple ));

        let shaders = [simple].iter().collect();

        ShaderManager{ active: Cell::new(None), shaders }
    }

    pub fn bind(&mut self, gl: &GL, type_: &ShaderType) {
        if let Some(shader) = self.shaders.iter().find(
            |shader|{ shader.type_() == *type_ }) {
            console::log_1(&JsValue::from(format!("Binding shader: {:?}", type_)));
            gl.use_program(Some(shader.program()));
            self.active = Cell::new(Some(shader));
        }
    }

    pub fn unbind(&self, gl: &GL) {
        if let Some(shader) = self.active.get() {
            console::log_1(&JsValue::from(format!("Binding shader: {:?}", shader.type_())));
            gl.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
        }
    }
}

