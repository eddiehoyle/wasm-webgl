use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

pub trait Buffer {
    fn buffer_vao();
    fn buffer_indices_u32(gl: &GL, indices: &[u32]);
    fn buffer_data_f32(gl: &GL, data: &[f32], attrib: u32, size: i32);
}

pub trait Draw {
    fn draw(&self, gl: &GL);
}