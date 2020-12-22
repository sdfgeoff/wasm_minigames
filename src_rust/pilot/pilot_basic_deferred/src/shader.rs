use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

/// The renderer needs to know where certain things are within the
/// shader so that the geometry can be rendered
pub struct VertexAttributes {
    pub positions: u32,
    pub normals: u32,
    pub uv0: u32,
}

impl VertexAttributes {
    pub fn new(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> Self {
        Self {
            positions: gl.get_attrib_location(&program, "vert_pos") as u32,
            normals: gl.get_attrib_location(&program, "vert_nor") as u32,
            uv0: gl.get_attrib_location(&program, "vert_uv0") as u32,
        }
    }
}

/// An error to represent problems with a shader.
#[derive(Debug)]
pub enum ShaderError {
    /// Call to gl.create_shader returned null
    ShaderAllocError,

    /// Call to create_program returned null
    ShaderProgramAllocError,

    ShaderCompileError {
        shader_type: u32,
        compiler_output: String,
    },
    /// Failed to receive error information about why the shader failed to compile
    /// Generally this is indicative of trying to get the error when one hasn't occured
    ShaderGetInfoError,

    /// I think this means that the Vertex and Fragment shaders incompatible
    ShaderLinkError(),
}

impl std::error::Error for ShaderError {}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub fn load_shader(
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

pub fn link_shaders(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, ShaderError> {
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
