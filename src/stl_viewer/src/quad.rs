use std::convert::TryInto;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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

pub struct Quad {
    position_buffer: WebGlBuffer,
    normal_buffer: WebGlBuffer,
    faces_buffer: WebGlBuffer,
    num_face_indices: u16,

    program: WebGlProgram,
    attrib_vertex_positions: u32,
    attrib_vertex_normals: u32,

    uniform_resolution: Option<WebGlUniformLocation>,
    uniform_time: Option<WebGlUniformLocation>,
    uniform_image_background: Option<WebGlUniformLocation>,
    uniform_image_matcap: Option<WebGlUniformLocation>,

    pub time: f32,
    pub resolution: (u32, u32),
    pub image_background: WebGlTexture,
    pub image_matcap: WebGlTexture,
}

impl Quad {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, QuadError> {
        let stl = include_bytes!("resources/monkey.stl");

        let (faces, positions, normals) = extact_buffers_from_stl(stl);

        let position_buffer = upload_f32_array(gl, positions)?;
        let normal_buffer = upload_f32_array(gl, normals)?;
        let num_face_indices = faces.len() as u16;
        let faces_buffer = upload_indices_array(gl, faces)?;

        let program = init_shader_program(
            gl,
            include_str!("resources/shader.vert"),
            include_str!("resources/shader.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "vert_pos") as u32;
        let attrib_vertex_normals = gl.get_attrib_location(&program, "vert_nor") as u32;

        let uniform_resolution = gl.get_uniform_location(&program, "iResolution");
        let uniform_time = gl.get_uniform_location(&program, "iTime");
        let uniform_image_background = gl.get_uniform_location(&program, "image_background");
        let uniform_image_matcap = gl.get_uniform_location(&program, "image_matcap");

        let image_background = load_texture(&gl, include_bytes!("resources/background.png"))
            .expect("Failed to load texture");
        let image_matcap = load_texture(&gl, include_bytes!("resources/matcap.png"))
            .expect("Failed to load texture");

        Ok(Self {
            position_buffer,
            normal_buffer,
            faces_buffer,
            num_face_indices,
            program,
            attrib_vertex_positions,
            attrib_vertex_normals,
            uniform_resolution,
            uniform_time,
            uniform_image_background,
            uniform_image_matcap,
            resolution: (100, 100),
            time: 0.0,
            image_background,
            image_matcap,
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));

        gl.uniform1f(self.uniform_time.as_ref(), self.time);
        gl.uniform2f(
            self.uniform_resolution.as_ref(),
            self.resolution.0 as f32,
            self.resolution.1 as f32,
        );

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_background,
            &self.image_background,
            TextureUnit::Unit0,
        );
        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_matcap,
            &self.image_matcap,
            TextureUnit::Unit1,
        );

        gl.enable_vertex_attrib_array(self.attrib_vertex_positions);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            self.attrib_vertex_positions,
            3, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );

        gl.enable_vertex_attrib_array(self.attrib_vertex_normals);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.normal_buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            self.attrib_vertex_normals,
            3, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );

        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.faces_buffer),
        );

        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.num_face_indices as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

fn get_f32(arr: &[u8], offset: u32) -> f32 {
    let offset = offset as usize;
    f32::from_le_bytes(arr[offset..offset + 4].try_into().unwrap())
}

fn extact_buffers_from_stl(stl: &[u8]) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    let mut faces = std::vec::Vec::new();
    let mut vertices = std::vec::Vec::new();
    let mut normals = std::vec::Vec::new();

    let num_faces = u32::from_le_bytes(stl[80..84].try_into().unwrap());

    for face in 0..(num_faces) {
        const STRIDE: u32 = 4 * 12 + 2;
        const OFFSET: u32 = 84;

        let face_offset = OFFSET + STRIDE * face;

        let nx = get_f32(stl, face_offset + 4 * 0);
        let ny = get_f32(stl, face_offset + 4 * 1);
        let nz = get_f32(stl, face_offset + 4 * 2);
        normals.push(nx);
        normals.push(ny);
        normals.push(nz);
        normals.push(nx);
        normals.push(ny);
        normals.push(nz);
        normals.push(nx);
        normals.push(ny);
        normals.push(nz);

        vertices.push(get_f32(stl, face_offset + 4 * 3));
        vertices.push(get_f32(stl, face_offset + 4 * 4));
        vertices.push(get_f32(stl, face_offset + 4 * 5));

        vertices.push(get_f32(stl, face_offset + 4 * 6));
        vertices.push(get_f32(stl, face_offset + 4 * 7));
        vertices.push(get_f32(stl, face_offset + 4 * 8));

        vertices.push(get_f32(stl, face_offset + 4 * 9));
        vertices.push(get_f32(stl, face_offset + 4 * 10));
        vertices.push(get_f32(stl, face_offset + 4 * 11));

        faces.push((face * 3) as u16);
        faces.push((face * 3 + 1) as u16);
        faces.push((face * 3 + 2) as u16);
    }

    (faces, vertices, normals)
}

fn upload_f32_array(
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

fn upload_indices_array(
    gl: &WebGl2RenderingContext,
    indices: Vec<u16>,
) -> Result<WebGlBuffer, QuadError> {
    let index_buffer = gl.create_buffer().ok_or(QuadError::BufferCreationFailed)?;
    gl.bind_buffer(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&index_buffer),
    );

    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&memory_buffer)
        .subarray(indices_location, indices_location + indices.len() as u32);

    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    Ok(index_buffer)
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
