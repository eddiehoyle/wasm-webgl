use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};
use std::collections::HashMap;

pub struct BufferF32 {
    array: js_sys::Float32Array,
    size: i32, // The size of each 'chunk' in the buffer
    data_type: u32,
}

impl BufferF32 {

    pub fn new(slice: &[f32], size: i32) -> Self {
        let mem = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let slice_location = slice.as_ptr() as u32 / 4;
        BufferF32{ array: js_sys::Float32Array::new(&mem)
            .subarray(slice_location, slice_location + slice.len() as u32),
            size,
            data_type: WebGl2RenderingContext::FLOAT,
        }
    }

    pub fn array(&self) -> &js_sys::Float32Array {
        &self.array
    }

    pub fn data_type(&self) -> u32 {
        self.data_type
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}


pub struct BufferU32 {
    array: js_sys::Uint32Array,
    data_type: u32,
}

impl BufferU32 {

    pub fn new(slice: &[u32]) -> Self {
        let mem = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let slice_location = slice.as_ptr() as u32 / 4;
        BufferU32{ array: js_sys::Uint32Array::new(&mem)
            .subarray(slice_location, slice_location + slice.len() as u32),
            data_type: WebGl2RenderingContext::UNSIGNED_INT,
        }
    }

    pub fn array(&self) -> &js_sys::Uint32Array {
        &self.array
    }

    pub fn data_type(&self) -> u32 {
        self.data_type
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let a = BufferF32::new(&[1.0, 2.0], 2);
        let b = a.array().length();
        assert_eq!(b, 2);
    }
}