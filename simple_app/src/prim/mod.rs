use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::render::traits::*;

pub struct Rectangle {
    x: u32,
    y: u32,
    indices: js_sys::Uint32Array,
    vertices: js_sys::Float32Array,
    vao: WebGlVertexArrayObject,

}
impl Rectangle {
    pub fn new(gl: &GL) -> Self {
        let f_mem = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let i_mem = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        info!("New Rectangle!");
        Rectangle { x: 0, y: 0,
            indices: js_sys::Uint32Array::new(&f_mem),
            vertices: js_sys::Float32Array::new(&i_mem),
            vao: gl.create_vertex_array().unwrap() }
    }
}

impl Buffer for Rectangle {
    fn bind(&self, gl: &GL) {
        gl.bind_vertex_array(Some(&self.vao));
    }
    fn buffer_indices_u32(&self, gl: &GL) {
        let id = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&id));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &self.indices,
            GL::STATIC_DRAW);

    }
    fn buffer_data_f32(&self, gl: &GL) {
        let id = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&id));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &self.vertices,
            GL::STATIC_DRAW,
        );
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    }
}