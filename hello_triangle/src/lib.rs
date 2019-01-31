#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

extern crate wasm_webgl_common;
use wasm_webgl_common::shader::Shader;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext};
use std::collections::HashMap;


pub fn create_buffer(slice: &[f32]) -> js_sys::Float32Array {
    let mem = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let slice_location = slice.as_ptr() as u32 / 4;
    js_sys::Float32Array::new(&mem)
        .subarray(slice_location, slice_location + slice.len() as u32)
}

pub fn bind_buffer(context: &WebGl2RenderingContext, index: u32, array: &js_sys::Float32Array) {
    let buffer = context.create_buffer().ok_or("failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &array,
        WebGl2RenderingContext::STATIC_DRAW,
    );
    context.vertex_attrib_pointer_with_i32(index, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(index);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vertex_source = r#"#version 300 es
        precision mediump float;

        in vec3 position;
        in vec3 color;

        out vec3 v_color;

        void main() {
            v_color = color;
            gl_Position = position;
        }
    "#;
    let fragment_source = r#"#version 300 es
        precision mediump float;
        in vec3 v_color;
        out vec4 outColor;
        void main() {
            outColor = vec4(v_color.xyz, 1.0);
        }
    "#;
    let simple_attributes: HashMap<u32, &str> = [(0, "position"), (1, "color")].iter().cloned().collect();
    let simple_shader = Shader::new(&context, &vertex_source, &fragment_source, simple_attributes);
    simple_shader.bind(&context);

    let vert_array = create_buffer(&[-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0]);
    let color_array = create_buffer(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);

    bind_buffer(&context,0, &vert_array);
    bind_buffer(&context,1, &color_array);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (vert_array.length() / 3) as i32,
    );

    simple_shader.unbind(&context);

    Ok(())
}