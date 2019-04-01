use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use js_sys::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct RenderSystemBuilder {
    canvas: Option<HtmlCanvasElement>,
    descriptions: Vec<ShaderDescription>,
}


struct RenderSystem {
    gl: GL,
}


impl RenderSystemBuilder {
    fn new() -> Self {
        Self::default()
    }
    fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.canvas = Some(canvas);
        self
    }
    fn register(mut self, description: ShaderDescription) -> Self {
        self.descriptions.push(description);
        self
    }
    fn build(self) -> Result<RenderSystem, String> {
        if let Some(canvas) = self.canvas {
            let gl = canvas
                .get_context("webgl2")
                .map_err(|_| "Unable to get rendering context")?
                .ok_or("Unable to get rendering context")?
                .dyn_into::<GL>()
                .map_err(|_| "Unable to get rendering context")?;

//            let mut renderers = HashMap::default();
//
//            for description in self.descriptions {
//                if renderers.contains_key(&description.name) {
//                    return Err(
//                        format!("Multiple renderers registered with id {}", description.name)
//                            .to_owned(),
//                    );
//                }
//
//                let renderer_id = description.name.clone();
//                let renderer = Self::compile(&gl, description)?;
//                renderers.insert(renderer_id, renderer);
//            }

//            for definition in self.definitions {
//                if renderers.contains_key(&definition.id) {
//                    return Err(
//                        format!("Multiple renderers registered with id {}", definition.id)
//                            .to_owned(),
//                    );
//                }
//
//                let renderer_id = definition.id.clone();
//                let renderer = Self::compile(&gl, definition)?;
//                renderers.insert(renderer_id, renderer);
//            }
//
//            gl.clear_color(0.0, 0.0, 0.0, 1.0);
//            gl.clear(GL::COLOR_BUFFER_BIT);
//            gl.enable(GL::DEPTH_TEST);
//            gl.depth_func(GL::LEQUAL);

            Ok(RenderSystem { gl })
        } else {
            Err("No canvas specified".to_owned())
        }
    }
    fn compile(gl: &GL, description: ShaderDescription) -> Result<(), String> {

        info!("Compiling render {}", description.name);
        let vert_shader = Self::compile_shader(
            gl,
            GL::VERTEX_SHADER,
            &description.vertex_source,
        )?;

        let frag_shader = Self::compile_shader(
            gl,
            GL::FRAGMENT_SHADER,
            &description.fragment_source,
        )?;

        let program = Self::link_program(gl, [vert_shader, frag_shader].iter())?;

//        let projection_matrix_location = gl
//            .get_uniform_location(&program, "uProjectionMatrix")
//            .ok_or("Unable to get uniform location for uProjectionMatrix")?;
//
//        let model_view_matrix_location = gl
//            .get_uniform_location(&program, "uModelViewMatrix")
//            .ok_or("Unable to get uniform location for uModelViewMatrix")?;


        let vao = gl
            .create_vertex_array()
            .ok_or("Unable to create vertex array".to_owned())?;

        let buffer = gl
            .create_buffer()
            .ok_or("Unable to create buffer".to_owned())?;

//        let attributes = &description.attributes
//            .iter()
//            .map(|attr|{
//                gl.enable_vertex_attrib_array(attr.location);
//                gl.bind_buffer(attr.buffer_type, Some(&buffer));
//                gl.vertex_attrib_pointer_with_i32(
//                    attr.location,
//                    attr.num_components,
//                    attr.buffer_data_type,
//                    false,
//                    0,
//                    0,
//                );
//                ShaderAttribute {
//                    attr.name,
//
//
//                }
//            }).collect::Vec<ShaderAttribute>();

        let uniforms = &description.uniforms
            .iter()
            .map(|uniform|{
                let location = gl
                    .get_uniform_location(&program, uniform.name.as_str())
                    .expect(format!("Unable to get uniform location for {}", uniform.name.as_str())
                        .as_str());
                ShaderUniform {
                    name: uniform.name.clone(),
                    location,
                    uniform_type: uniform.uniform_type.clone()
                }
            })
            .collect::<Vec<ShaderUniform>>();


//        gl.bind_vertex_array(Some(&vao));

//        for input in &definition.inputs {
//            gl.enable_vertex_attrib_array(input.location);
//            gl.bind_buffer(input.buffer_type, Some(&buffer));
//            gl.vertex_attrib_pointer_with_i32(
//                input.location,
//                input.num_components,
//                input.buffer_data_type,
//                false,
//                0,
//                0,
//            );
//        }

        gl.bind_vertex_array(None);
        Ok(())

//        Ok(Renderable {
//            definition,
//            program,
//            vao,
//            projection_matrix_location,
//            model_view_matrix_location,
//        })
    }

    fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
            {
                Ok(shader)
            } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into()))
        }
    }

    fn link_program<'a, T>(gl: &GL, shaders: T) -> Result<WebGlProgram, String>
        where
            T: IntoIterator<Item = &'a WebGlShader> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        for shader in shaders {
            gl.attach_shader(&program, shader)
        }
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
            {
                Ok(program)
            } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating program object".into()))
        }
    }
}




/// Current problem:
/// Need a simple way of describing a shader's source,
/// attributes and uniforms to RenderSystem builder



#[derive(Debug)]
struct ShaderDescription {
    name: String,
    vertex_source: String,
    fragment_source: String,
    attributes: Vec<ShaderAttribute>,
    uniforms: Vec<ShaderUniform>,
}

#[derive(Debug)]
struct ShaderDefinition {
    name: String,
    program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    attributes: Vec<ShaderAttribute>,
    uniforms: Vec<ShaderUniform>,
}

#[derive(Debug)]
struct ShaderAttribute {
    name: String,
    location: u32,
    buffer_type: u32,
    buffer_data_type: u32,
    num_components: i32,
}

#[derive(Debug)]
struct ShaderUniform {
    name: String,
    location: WebGlUniformLocation,
    uniform_type: u32,
}

struct Renderer {
    shader: ShaderDefinition,

}
