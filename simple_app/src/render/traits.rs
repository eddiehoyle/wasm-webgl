use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;

pub trait Buffer {
    fn bind(&self, gl: &GL);
    fn buffer_indices_u32(&self, gl: &GL);
    fn buffer_data_f32(&self, gl: &GL);
}

pub trait Draw {
    fn draw(&self, gl: &GL);
}