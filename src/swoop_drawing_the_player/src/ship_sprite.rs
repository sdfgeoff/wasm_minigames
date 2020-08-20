use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation,
};

use super::texture::{bind_2d_texture_to_uniform, load_texture, TextureUnit};

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

/// An error with this whole object.
#[derive(Debug)]
pub enum QuadError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed,

    /// An unhandled/unspecified error
    JsError(JsValue),

    /// Something wrong with the shader
    ShaderError(ShaderError),
}

impl From<JsValue> for QuadError {
    fn from(err: JsValue) -> QuadError {
        QuadError::JsError(err)
    }
}

impl From<ShaderError> for QuadError {
    fn from(err: ShaderError) -> QuadError {
        QuadError::ShaderError(err)
    }
}

pub struct ShipSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_ship_engine: Option<WebGlUniformLocation>,
    uniform_ship_texture: Option<WebGlUniformLocation>,
    uniform_ship_color: Option<WebGlUniformLocation>,

    uniform_world_to_camera: Option<WebGlUniformLocation>,
    uniform_world_to_sprite: Option<WebGlUniformLocation>,
    uniform_camera_to_clipspace: Option<WebGlUniformLocation>,

    pub ship_texture: WebGlTexture,
    pub ship_color: (f32, f32, f32, f32),
    pub ship_engine: f32,

    pub world_to_camera: [f32; 9],
    pub world_to_sprite: [f32; 9],
    pub camera_to_clipspace: [f32; 9],
}

impl ShipSprite {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, QuadError> {
        let position_buffer =
            upload_array_f32(gl, vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0])?;

        let program = init_shader_program(
            gl,
            include_str!("resources/ship.vert"),
            include_str!("resources/ship.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_ship_texture = gl.get_uniform_location(&program, "ship_texture");
        let uniform_ship_engine = gl.get_uniform_location(&program, "ship_engine");
        let uniform_ship_color = gl.get_uniform_location(&program, "ship_color");

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_sprite = gl.get_uniform_location(&program, "world_to_sprite");
        let uniform_camera_to_clipspace = gl.get_uniform_location(&program, "camera_to_clipspace");

        let ship_texture = load_texture(&gl, include_bytes!("resources/ship.png"))
            .expect("Failed to load texture");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_ship_engine,
            uniform_ship_texture,
            uniform_ship_color,

            uniform_world_to_camera,
            uniform_world_to_sprite,
            uniform_camera_to_clipspace,

            ship_color: (1.0, 1.0, 1.0, 1.0),
            ship_texture,
            ship_engine: 0.0,

            world_to_camera: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            world_to_sprite: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            camera_to_clipspace: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));

        gl.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE);

        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_world_to_sprite.as_ref(),
            true,
            &self.world_to_sprite,
        );
        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_world_to_camera.as_ref(),
            true,
            &self.world_to_camera,
        );
        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_camera_to_clipspace.as_ref(),
            true,
            &self.camera_to_clipspace,
        );

        gl.uniform4f(
            self.uniform_ship_color.as_ref(),
            self.ship_color.0,
            self.ship_color.1,
            self.ship_color.2,
            self.ship_color.3,
        );
        gl.uniform1f(self.uniform_ship_engine.as_ref(), self.ship_engine);

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_ship_texture,
            &self.ship_texture,
            TextureUnit::Unit0,
        );

        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        gl.vertex_attrib_pointer_with_i32(
            self.attrib_vertex_positions,
            2, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );
        gl.enable_vertex_attrib_array(self.attrib_vertex_positions);

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}

fn upload_array_f32(
    gl: &WebGl2RenderingContext,
    vertices: Vec<f32>,
) -> Result<WebGlBuffer, QuadError> {
    let position_buffer = gl.create_buffer().ok_or(QuadError::BufferCreationFailed)?;

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
