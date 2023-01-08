use glow::{Context, HasContext};

use super::shader::Shader;

#[derive(Debug)]
pub enum ShaderProgramError {
    ShaderProgramAllocError(String),
    ShaderProgramLinkError(String),
}

pub struct ShaderProgram {
    pub program: glow::Program,
    pub attrib_vertex_positions: u32,
    pub uniforms: std::collections::HashMap<String, glow::UniformLocation>,
}

impl ShaderProgram {
    pub fn new(
        gl: &Context,
        vert_shader: &Shader,
        frag_shader: &Shader,
        uniform_names: Vec<String>,
    ) -> Result<Self, ShaderProgramError> {
        let program = unsafe { init_shader_program(gl, vert_shader, frag_shader)? };
        let attrib_vertex_positions = unsafe {
            gl.get_attrib_location(program, "aVertexPosition")
                .expect("No vertx positions?")
        };

        let mut uniforms = std::collections::HashMap::with_capacity(uniform_names.len());

        for uniform_name in uniform_names {
            let uniform_location = unsafe { gl.get_uniform_location(program, &uniform_name) };
            if let Some(loc) = uniform_location {
                uniforms.insert(uniform_name, loc);
            }
        }

        Ok(Self {
            program,
            attrib_vertex_positions,
            uniforms,
        })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}

pub unsafe fn init_shader_program(
    gl: &Context,
    vert_shader: &Shader,
    frag_shader: &Shader,
) -> Result<glow::Program, ShaderProgramError> {
    let shader_program = gl
        .create_program()
        .map_err(ShaderProgramError::ShaderProgramAllocError)?;
    gl.attach_shader(shader_program, vert_shader.shader);
    gl.attach_shader(shader_program, frag_shader.shader);

    gl.link_program(shader_program);

    if !(gl.get_program_link_status(shader_program)) {
        let compiler_output = gl.get_program_info_log(shader_program);
        gl.delete_program(shader_program);
        return Err(ShaderProgramError::ShaderProgramLinkError(compiler_output));
    }

    Ok(shader_program)
}
