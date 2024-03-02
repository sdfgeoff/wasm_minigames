use glow::{Context, HasContext, FRAGMENT_SHADER, VERTEX_SHADER};

#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
}
impl ShaderType {
    pub fn to_gl_constant(&self) -> u32 {
        match self {
            ShaderType::Vertex => VERTEX_SHADER,
            ShaderType::Fragment => FRAGMENT_SHADER,
        }
    }
}

#[derive(Debug)]
pub enum ShaderError {
    ShaderAllocError(String),
    ShaderCompileError {
        shader_type: ShaderType,
        compiler_output: String,
        shader_text: String,
    },
}

pub struct Shader {
    pub shader: glow::Shader,
}

impl Shader {
    pub fn new(
        gl: &Context,
        shader_type: ShaderType,
        shader_text: &str,
    ) -> Result<Self, ShaderError> {
        let shader = unsafe { load_shader(gl, shader_type, shader_text)? };
        Ok(Self { shader })
    }
}

unsafe fn load_shader(
    gl: &Context,
    shader_type: ShaderType,
    shader_text: &str,
) -> Result<glow::Shader, ShaderError> {
    let shader = gl
        .create_shader(shader_type.to_gl_constant())
        .map_err(ShaderError::ShaderAllocError)?;

    gl.shader_source(shader, shader_text);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        let compiler_output = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(ShaderError::ShaderCompileError {
            shader_type,
            compiler_output: compiler_output,
            shader_text: shader_text.to_string(),
        });
    }
    Ok(shader)
}
