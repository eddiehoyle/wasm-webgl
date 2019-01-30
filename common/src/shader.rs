use wasm_bindgen::prelude::{JsValue};
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

pub struct Shader {
    program: WebGlProgram,
}

impl Shader {

    /// Bind shader
    fn bind(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program))
    }

    /// Unbind shader
    fn unbind(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&WebGlProgram::from(JsValue::NULL)));
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

        // TODO: Break this out
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

}
