use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};
use std::collections::HashMap;

pub struct BufferF32 {
    array: js_sys::Float32Array,
}


//impl From<&[f32]> for BufferF32 {
//    fn from(slice: &[f32]) -> Self {
//        let mem = wasm_bindgen::memory()
//            .dyn_into::<WebAssembly::Memory>()
//            .unwrap()
//            .buffer();
//        let slice_location = slice.as_ptr() as u32 / 4;
//        BufferF32 { array: js_sys::Float32Array::new(&mem)
//            .subarray(slice_location, slice_location + slice.len() as u32) }
//    }
//}
//
//impl From<&[u32]> for Buffer<js_sys::Uint32Array> {
//    fn from(slice: &[u32]) -> Self {
//        let mem = wasm_bindgen::memory()
//            .dyn_into::<WebAssembly::Memory>()
//            .unwrap()
//            .buffer();
//        let slice_location = slice.as_ptr() as u32 / 4;
//        Buffer::<js_sys::Uint32Array>{
//            array: js_sys::Uint32Array::new(&mem)
//                .subarray(slice_location, slice_location + slice.len() as u32) }
//    }
//}

impl BufferF32 {

    pub fn new(slice: &[f32]) -> Self {
        let mem = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let slice_location = slice.as_ptr() as u32 / 4;
        BufferF32{ array: js_sys::Float32Array::new(&mem)
            .subarray(slice_location, slice_location + slice.len() as u32)
        }
    }

    pub fn array(&self) -> &js_sys::Float32Array {
        &self.array
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let a = BufferF32::new(&[1.0, 2.0]);
        let b = a.array().length();
        assert_eq!(b, 2);
    }
}