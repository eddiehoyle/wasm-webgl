use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use js_sys::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::collections::HashSet;
use std::collections::HashMap;
use specs::prelude::*;

pub struct RenderSystem {
    gl: GL,
    renderers: Vec<Renderer>,
}


impl<'a> System<'a> for RenderSystem {

    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        info!("Setting up RenderSystem");
    }
}


#[derive(Debug, Default)]
pub struct RenderSystemBuilder {
    canvas: Option<HtmlCanvasElement>,
    descriptions: Vec<ShaderDescription>,
}



impl RenderSystemBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.canvas = Some(canvas);
        self
    }
    pub fn register(mut self, description: ShaderDescription) -> Self {
        self.descriptions.push(description);
        self
    }
    pub fn build(self) -> Result<RenderSystem, String> {
        if let Some(canvas) = self.canvas {
            let gl = canvas
                .get_context("webgl2")
                .map_err(|_| "Unable to get rendering context")?
                .ok_or("Unable to get rendering context")?
                .dyn_into::<GL>()
                .map_err(|_| "Unable to get rendering context")?;

            let mut renderers = Vec::new();
            for description in &self.descriptions {
                let definition = Self::compile(&gl, description.clone())?;
                renderers.push(Renderer {shader: definition});
            }

            Ok(RenderSystem { gl, renderers })
        } else {
            Err("No canvas specified".to_owned())
        }
    }
    fn compile(gl: &GL, mut description: ShaderDescription) -> Result<ShaderDefinition, String> {

        info!("Compiling shader {}", &description.name);
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

        info!("Linked program {}", &description.name);

        for uniform in &mut description.uniforms {
            uniform.location = Some(gl
                .get_uniform_location(&program, uniform.name.as_str())
                .expect(format!("Unable to get uniform location for {}", uniform.name.as_str())
                    .as_str()));
        }

        for attribute in &mut description.attributes {
            let location = gl
                .get_attrib_location(&program, attribute.name.as_str());
            if location == -1 {
                return Err(format!("Unable to get attribute location for {}", attribute.name.as_str()));
            }
            attribute.location = Some(location);
        }

        Ok(ShaderDefinition {
            name: description.name,
            program,
            uniforms: description.uniforms,
            attributes: description.attributes,
        })
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



#[derive(Debug, Clone)]
pub struct ShaderDescription {
    pub name: String,
    pub vertex_source: String,
    pub fragment_source: String,
    pub attributes: Vec<ShaderAttribute>,
    pub uniforms: Vec<ShaderUniform>,
}

#[derive(Debug, Clone)]
pub struct ShaderDefinition {
    pub name: String,
    pub program: WebGlProgram,
    pub attributes: Vec<ShaderAttribute>,
    pub uniforms: Vec<ShaderUniform>,
}

#[derive(Debug, Clone)]
pub struct ShaderAttribute {
    pub name: String,
    pub location: Option<i32>,
    pub buffer_type: u32,
    pub buffer_data_type: u32,
    pub num_components: i32,
}

#[derive(Debug, Clone)]
pub struct ShaderUniform {
    pub name: String,
    pub location: Option<WebGlUniformLocation>,
    pub uniform_type: u32,
}

pub struct Renderer {
    pub shader: ShaderDefinition,
}