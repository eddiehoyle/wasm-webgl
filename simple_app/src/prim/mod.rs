use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::render::traits::*;

pub mod camera;

pub struct Rectangle {
    x: u32,
    y: u32,
    indices: js_sys::Uint32Array,
    vertices: js_sys::Float32Array,
    vao: WebGlVertexArrayObject,
}


impl Rectangle {
    pub fn new(gl: &GL, width: f32, height: f32) -> Self {

        let indices : [u32; 6] = [
            0, 1, 3,
            3, 1, 2,
        ];
//        let vertices : [f32; 12] = [
//            -0.5, 0.5, 0.0,
//            -0.5, -0.5, 0.0,
//            0.5, -0.5, 0.0,
//            0.5, 0.5, 0.0,
//        ];

        let vertices : [f32; 12] = [
            0.0,   height, -20.0,
            0.0,   0.0,    -20.0,
            width, 0.0,    -20.0,
            width, height, -20.0,
        ];

        let f_mem = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let i_mem = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let indices_location = indices.as_ptr() as u32 / 4;
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let indices = js_sys::Uint32Array::new(&f_mem)
            .subarray(indices_location, indices_location + indices.len() as u32);
        let vertices = js_sys::Float32Array::new(&f_mem)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);
        debug!("New Rectangle!");

        let rect = Rectangle { x: 0, y: 0,
            indices,
            vertices,
            vao: gl.create_vertex_array().unwrap() };
        rect.bind(gl);
        rect.buffer_indices_u32(gl);
        rect.buffer_data_f32(gl);
        rect
    }
}

impl Draw for Rectangle {
    fn draw(&self, gl: &GL) {
        gl.draw_elements_with_i32(GL::TRIANGLES,
                                  self.indices.length() as i32,
                                  GL::UNSIGNED_INT,
                                  0);
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
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    }
}