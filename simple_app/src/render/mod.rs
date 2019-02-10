use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;



mod shader;

pub struct WebRenderer {
}

impl WebRenderer {
    pub fn new(gl: Rc<GL>) -> WebRenderer {
        console::log_1(&JsValue::from("new WebRenderer"));

        static SIMLPE_VS: &'static str = include_str!("./../../dist/static-vertex.glsl");
        static SIMPLE_FS: &'static str = include_str!("./../../dist/static-fragment.glsl");

        let s = shader::Shader::new(gl.borrow(),
                                    SIMLPE_VS,
                                    SIMPLE_FS,
                                    &[],
                                    &[]);
        if s.is_err() {
            console::log_1(&JsValue::from(s.err().unwrap()));
        }
        WebRenderer {}
    }
}
