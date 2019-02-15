use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::cmp::Ordering;

type UniformMap = HashMap<String, WebGlUniformLocation>;
type AttributeMap = HashMap<String, i32>;

pub mod manager;

#[derive(Ord, PartialOrd, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderType {
    Simple,
    Persp,
}

pub struct Shader  {
    program_: WebGlProgram,
    uniforms_: UniformMap,
    attributes_: AttributeMap,
    type_: ShaderType,
}

impl Shader {
    pub fn new(gl: &GL,
               vert_source: &str,
               frag_source: &str,
               attributes: &[&str],
               uniforms: &[&str],
               type_: ShaderType, ) -> Result<Shader, JsValue> {

        let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source)?;
        let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        let mut uniforms_map: UniformMap = HashMap::new();
        for uniform in uniforms {
            uniforms_map.insert(uniform.to_string(),
                                gl.get_uniform_location(&program, uniform)
                                    .expect(&format!("Uniform '{}' not found", uniform)));
        }

        let mut attributes_map: AttributeMap = HashMap::new();
        for attribute in attributes {
            attributes_map.insert(attribute.to_string(),
                                  gl.get_attrib_location(&program, attribute));
        }
        Ok(Shader { program_: program,
            uniforms_: uniforms_map,
            attributes_: attributes_map,
            type_: type_,
        })
    }

    pub fn program(&self) -> &WebGlProgram {
        &self.program_
    }

    pub fn type_(&self) -> ShaderType {
        self.type_
    }

    pub fn attributes(&self, gl: &GL) -> &AttributeMap {
        &self.attributes_
    }
}

fn compile_shader(gl: &GL,
                  shader_type: u32,
                  source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error compiling shader shader".to_string()))
    }
}

fn link_program(gl: &GL,
                vert_shader: &WebGlShader,
                frag_shader: &WebGlShader
) -> Result<WebGlProgram, String> {
    let program = gl.create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);

    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
        {
            Ok(program)
        } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating shader program".to_string()))
    }
}
