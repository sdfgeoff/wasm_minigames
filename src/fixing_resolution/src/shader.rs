use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};


pub trait Shader {
    fn program() -> WebGlProgram,
    fn attrib_vertex_positions() -> u32,
}


#[derive(Debug)]
pub enum ShaderError {
    ShaderAllocError,
    ShaderProgramAllocError,
    ShaderGetInfoError,
    MissingUniform(String),
    ShaderCompileError {
        shader_type: u32,
        compiler_output: String,
    },
    ShaderLinkError(),
}


pub struct SimpleShader {
	pub program: WebGlProgram,
    pub attrib_vertex_positions: u32,
}

impl SimpleShader {
	pub fn new(gl: &WebGl2RenderingContext, vert: &str, frag: &str) -> Result<Self, ShaderError> {
        let program = init_shader_program(
            gl,
            vert,
            frag,
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition");

        Ok(Self {
            program,
            attrib_vertex_positions: attrib_vertex_positions as u32,
        })
    }
}




fn load_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    shader_text: &str,
) -> Result<WebGlShader, ShaderError> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or(ShaderError::ShaderAllocError)?;
    gl.shader_source(&shader, shader_text);
    gl.compile_shader(&shader);
    if !gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .is_truthy()
    {
        let compiler_output = &gl
            .get_shader_info_log(&shader)
            .ok_or(ShaderError::ShaderGetInfoError)?;
        gl.delete_shader(Some(&shader));
        return Err(ShaderError::ShaderCompileError {
            shader_type,
            compiler_output: compiler_output.to_string(),
        });
    }
    Ok(shader)
}


pub fn init_shader_program(
    gl: &WebGl2RenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, ShaderError> {
    let vert_shader = load_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vert_source)?;
    let frag_shader = load_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_source)?;

    let shader_program = gl
        .create_program()
        .ok_or(ShaderError::ShaderProgramAllocError)?;
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);

    gl.link_program(&shader_program);

    if !(gl.get_program_parameter(&shader_program, WebGl2RenderingContext::LINK_STATUS)).is_truthy()
    {
        gl.delete_program(Some(&shader_program));
        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));
        return Err(ShaderError::ShaderLinkError());
    }

    Ok(shader_program)
}
