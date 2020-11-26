use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader};

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
    ShaderLinkError(String),

    /// Failed to create buffer to upload data into
    BufferCreationFailed,

    /// Generic javascript error
    JsError(JsValue),
}

impl From<JsValue> for ShaderError {
    fn from(err: JsValue) -> ShaderError {
        ShaderError::JsError(err)
    }
}

pub fn upload_array_f32(
    gl: &WebGl2RenderingContext,
    vertices: Vec<f32>,
) -> Result<WebGlBuffer, ShaderError> {
    let position_buffer = gl
        .create_buffer()
        .ok_or(ShaderError::BufferCreationFailed)?;

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let vertices_location = vertices.as_ptr() as u32 / 4;

    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    Ok(position_buffer)
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
        let data = ShaderError::ShaderLinkError(
            gl.get_program_info_log(&shader_program).or(Some("No Error".to_string())).unwrap()
        );
        gl.delete_program(Some(&shader_program));
        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));
        return Err(data);
    }

    Ok(shader_program)
}
