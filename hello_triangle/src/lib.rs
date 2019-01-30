extern crate wasm_webgl_common;
use wasm_webgl_common::shader::Shader;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext};


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
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = Shader::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"
        precision highp float;
        attribute vec4 position;
        attribute vec3 color;
        varying vec3 v_color;
        void main() {
            v_color = color;
            gl_Position = position;
        }
    "#,
    )?;
    let frag_shader = Shader::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"
        precision highp float;
        varying vec3 v_color;
        void main() {
            gl_FragColor = vec4(v_color, 1.0);
        }
    "#,
    )?;
    let program = Shader::link_program(&context, [vert_shader, frag_shader].iter())?;
    context.use_program(Some(&program));

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
    Ok(())
}