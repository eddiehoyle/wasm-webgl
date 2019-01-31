use wasm_bindgen::prelude::{JsValue};
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};
use std::collections::HashMap;

use std::iter::Iterator;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[cfg(test)]
mod tests {
    #[test]
    fn shader() {
        let vertex_source = r#"#version 300 es
            in vec4 position;
            void main() {
                gl_Position = position;
            }
        "#;
        let fragment_source = r#"#version 300 es
            precision mediump float;
            out vec4 outColor;
            void main() {
               outColor = vec4(0.0, 0.0, 0.0, 1.0);
            }
        "#;
//        let document = web_sys::window().unwrap().document().unwrap();
    }
//    let canvas = document.get_element_by_id("canvas").unwrap();
//    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
}

pub struct Shader<'a>  {
    program: WebGlProgram,
    attributes: HashMap<u32, &'a str>,
}

impl<'a> Shader<'a> {

    /// TODO
    pub fn new(context: &WebGl2RenderingContext,
              vertex_source: &str,
              fragment_source: &str,
              attributes: HashMap<u32, &'a str>) -> Self {
        let vertex_shader = compile_shader(context,
                                           WebGl2RenderingContext::VERTEX_SHADER,
                                           vertex_source).expect("Error compiling vertex shader");
        let fragment_shader = compile_shader(context,
                                             WebGl2RenderingContext::FRAGMENT_SHADER,
                                             fragment_source).expect("Error compiling fragment shader");

        let program = link_program(&context,
                                   [vertex_shader, fragment_shader].iter(),
                                   &attributes).unwrap();

        Shader { program, attributes, }
    }

    /// Bind shader
    fn bind(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program))
    }

    /// Unbind shader
    fn unbind(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
    }

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

pub fn link_program<'a, S>(
    context: &WebGl2RenderingContext,
    shaders: S,
    attributes: &HashMap<u32, &'a str>,
) -> Result<WebGlProgram, String>
    where
        S: Iterator<Item=&'a WebGlShader> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }

    for (index, name) in attributes {
        context.bind_attrib_location(&program, *index, *name);
    }

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
