#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};



#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
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
    let frag_shader = compile_shader(
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
    let program = link_program(&context, [vert_shader, frag_shader].iter())?;
    context.use_program(Some(&program));

    let vertices: [f32; 12] = [
        -0.5, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.5, 0.5, 0.0,
    ];

    let indices: [u32; 6] = [
        0, 1, 3,
        3, 1, 2
    ];

    let uvs: [f32; 8] = [
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
    ];

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));

    let vbo = context.create_buffer().ok_or("Failed to create VBO").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vbo));
    let vbo_mem = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let vbo_array = js_sys::Int32Array::new(&vbo_mem)
        .subarray(indices.as_ptr() as u32,
                  indices.as_ptr() as u32 + indices.len() as u32);
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vbo));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &vbo_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // -------------------------------------------------

    let vbo_vertices = context.create_buffer().ok_or("Failed to create VBO").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
    let vbo_vertices_mem = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let vbo_vertices_array = js_sys::Float32Array::new(&vbo_vertices_mem)
        .subarray(vertices.as_ptr() as u32,
                  vertices.as_ptr() as u32 + vertices.len() as u32);
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo_vertices));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &vbo_vertices_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&0));

//    GLuint vboID;
//    glGenBuffers( 1, &vboID );
//    glBindBuffer( GL_ARRAY_BUFFER, vboID );
//    glBufferData( GL_ARRAY_BUFFER, sizeof( GLfloat ) * data.size(), &data[0], GL_STATIC_DRAW );
//    glVertexAttribPointer( attributeNumber,   // The attribute number
//                           coordinateSize,    // Length of each data array
//                           GL_FLOAT,          // Type of data
//                           GL_FALSE,          // Is this data normalized
//                           0,                 // Distance between vertices (is there any other data between them?)
//                           0 );               // Offset. Should it start at the beginning of this data?
//    glBindBuffer( GL_ARRAY_BUFFER, 0 );
//    m_vbos.push_back( vboID );

//    context.buffer_data_with_array_buffer_view(
//        WebGl2RenderingContext::ARRAY_BUFFER,
//        &array,
//        WebGl2RenderingContext::STATIC_DRAW,
//    );
//    GLuint vboID;
//    glGenBuffers( 1, &vboID );
//    glBindBuffer( GL_ELEMENT_ARRAY_BUFFER, vboID );
//    glBufferData( GL_ELEMENT_ARRAY_BUFFER, sizeof( GLuint ) * indices.size(), &indices[0], GL_STATIC_DRAW );
//    m_vbos.push_back( vboID );
//



//
//    bind_buffer(&context,0, &vert_array);
//    bind_buffer(&context,1, &color_array);
//
//    context.clear_color(0.0, 0.0, 0.0, 1.0);
//    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
//
//    context.draw_arrays(
//        WebGl2RenderingContext::TRIANGLES,
//        0,
//        (vert_array.length() / 3) as i32,
//    );
    Ok(())
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGl2RenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.bind_attrib_location(&program, 0, "position");
    context.bind_attrib_location(&program, 1, "color");
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}

//pub fn create_buffer<DataT, ArrayT: Default>(slice: &[DataT]) -> ArrayT {
//    let mem = wasm_bindgen::memory()
//        .dyn_into::<WebAssembly::Memory>()
//        .unwrap()
//        .buffer();
//    let slice_location = slice.as_ptr() as u32 / 4;
//    let mut a = <ArrayT as Default>::default();
//    a.subarray(slice_location, slice_location + slice.len() as u32)
//}


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
