#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate wasm_webgl_common;

use wasm_webgl_common::shader::Shader;
use wasm_webgl_common::buffer::*;
use wasm_webgl_common::texture::*;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use std::mem;
use std::fs::File;

use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

pub fn buffer_array(context: &WebGl2RenderingContext,
                    index: u32,
                    buffer: &BufferF32,
) {
    let id = context.create_buffer().ok_or("failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&id));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &buffer.array(),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    context.vertex_attrib_pointer_with_i32(index, buffer.size(), buffer.data_type(), false, 0, 0);
}

pub fn buffer_indices(context: &WebGl2RenderingContext,
                      buffer: &BufferU32,
) {
    let id = context.create_buffer().ok_or("failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&id));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &buffer.array(),
        WebGl2RenderingContext::STATIC_DRAW,
    );
}


#[wasm_bindgen]
pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

//#[wasm_bindgen]
//pub fn read_image(img: &ImageData) {
//    console::log_1(&JsValue::from(format!("{:?}x{:?}", img.width(), img.height())));
//
//    let document = web_sys::window().unwrap().document().unwrap();
//    let canvas = document.get_element_by_id("canvas").unwrap();
//    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
//
//    // TODO: This looks like it's on the right path
//    // Try pass in the correct context instead of re-fetching
//    // Set up texture bind etc as before
//
//    let context = canvas
//        .get_context("webgl2")
//        .unwrap().unwrap()
//        .dyn_into::<WebGl2RenderingContext>().unwrap();
//
//    context.tex_image_2d_with_u32_and_u32_and_image_data(
//        WebGl2RenderingContext::TEXTURE_2D,
//        0,
//        WebGl2RenderingContext::RGBA as i32,
//        WebGl2RenderingContext::RGBA,
//        WebGl2RenderingContext::UNSIGNED_BYTE,
//        &img,
//    ).unwrap();
//    console::log_1(&JsValue::from("image loaded i thikn?"));
//}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let context = Rc::new(context);

    let vertex_source = r#"#version 300 es
        precision mediump float;

        in vec3 position;
        in vec2 texcoord;
        out vec2 v_texcoord;

        void main() {
            v_texcoord = texcoord;
            gl_Position = vec4(position, 1.0);
        }
    "#;
    let fragment_source = r#"#version 300 es
        precision mediump float;

        in vec2 v_texcoord;
        uniform sampler2D u_texture;
        out vec4 outColor;

        void main() {
            outColor = texture(u_texture, vec2(v_texcoord.x + 1.0, v_texcoord.y + 1.0));
        }
    "#;
    let simple_attributes: HashMap<u32, &str> = [(0, "position"), (1, "texcoord")].iter().cloned().collect();
    let simple_shader = Shader::new(&context,
                                    &"simple",
                                    &vertex_source,
                                    &fragment_source,
                                    simple_attributes)?;



    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));

    let vert_buffer = BufferF32::new(&[
        -0.5, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.5, 0.5, 0.0,
    ], 3);
//    let color_buffer = BufferF32::new(&[
//        1.0, 0.0, 0.0,
//        0.0, 1.0, 0.0,
//        0.0, 0.0, 1.0,
//        1.0, 1.0, 1.0,
//    ], 3);
    let texcoord_buffer = BufferF32::new(&[
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
    ], 2);

    let index_buffer = BufferU32::new(&[
        0, 1, 3,
        3, 1, 2,
    ]);


    buffer_indices(&context, &index_buffer);
    buffer_array(&context, 0, &vert_buffer);
    buffer_array(&context, 1, &texcoord_buffer);

    // ========================================================================================== //

    let temp_texture = BufferU8::new(&[0, 0, 255, 255], 2);
    let texture = context.create_texture().unwrap();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        1,
        1,
        0,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        Some(&temp_texture.array()),
    ).expect("Texture image 2d");

    // ========================================================================================== //

    console::log_1(&JsValue::from("Loading image..."));
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);
    let context_clone = Rc::clone(&context);

    let onload = Closure::wrap(Box::new(move || {
        console::log_1(&JsValue::from("Image loaded"));
        let texture = context_clone.create_texture().unwrap();
        context_clone.active_texture(WebGl2RenderingContext::TEXTURE0);
        context_clone.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
//        context_clone.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
        context_clone.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D,
                                     WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                     WebGl2RenderingContext::NEAREST as i32);
        context_clone.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D,
                                     WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                                     WebGl2RenderingContext::NEAREST as i32);
        context_clone.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image_clone.borrow(),
        ).expect("Texture image 2d");




        simple_shader.bind(&context_clone);

        context_clone.clear_color(0.0, 0.0, 0.0, 1.0);
        context_clone.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        context_clone.bind_vertex_array(Some(&vao));
        context_clone.enable_vertex_attrib_array(0);
        context_clone.enable_vertex_attrib_array(1);
        context_clone.active_texture(WebGl2RenderingContext::TEXTURE0);
        context_clone.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES,
                                       index_buffer.array().length() as i32,
                                       WebGl2RenderingContext::UNSIGNED_INT,
                                       0);
        context_clone.disable_vertex_attrib_array(0);
        context_clone.disable_vertex_attrib_array(1);

        simple_shader.unbind(&context_clone);



    }) as Box<dyn Fn()>);

    let image = image.borrow();
    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src("cat.png");
    onload.forget();

    // ========================================================================================== //

    Ok(())
}
