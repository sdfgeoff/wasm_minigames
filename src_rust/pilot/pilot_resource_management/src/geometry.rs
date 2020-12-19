use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::shader::VertexAttributes;

/// An error with this whole object.
#[derive(Debug)]
pub enum GeometryError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed,

    /// An unhandled/unspecified error
    JsError(JsValue),
}


impl std::error::Error for GeometryError {}

impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


pub struct BufferDef {
    /// Reference to the data on the GPU
    buffer: WebGlBuffer,

    /// How many components are there in the buffer
    buffer_length: i32,
}

pub struct Geometry {
    positions: BufferDef,
    normals: BufferDef,
    uv0: BufferDef,
    indices: BufferDef,
}

impl Geometry {
    pub fn new(
        gl: &WebGl2RenderingContext,
        positions: &Vec<f32>,
        normals: &Vec<f32>,
        uv0: &Vec<f32>,
        indices: &Vec<u16>,
    ) -> Result<Self, GeometryError> {
        Ok(Self {
            positions: BufferDef {
                buffer: upload_f32_array(gl, positions)?,
                buffer_length: positions.len() as i32,
            },
            normals: BufferDef {
                buffer: upload_f32_array(gl, normals)?,
                buffer_length: normals.len() as i32,
            },
            uv0: BufferDef {
                buffer: upload_f32_array(gl, uv0)?,
                buffer_length: normals.len() as i32,
            },
            indices: BufferDef {
                buffer: upload_indices_array(gl, indices)?,
                buffer_length: indices.len() as i32,
            },
        })
    }

    /// Mage sure everything is set up for rendering this geometry
    pub fn bind(&self, gl: &WebGl2RenderingContext, vertex_attributes: &VertexAttributes) {
        gl.enable_vertex_attrib_array(vertex_attributes.positions);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.positions.buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            vertex_attributes.positions,
            3, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );

        if vertex_attributes.normals != 0xFFFFFFFF {
            gl.enable_vertex_attrib_array(vertex_attributes.normals);
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&self.normals.buffer),
            );
            gl.vertex_attrib_pointer_with_i32(
                vertex_attributes.normals,
                3, // num components
                WebGl2RenderingContext::FLOAT,
                false, // normalize
                0,     // stride
                0,     // offset
            );
        }
        
        if vertex_attributes.uv0 != 0xFFFFFFFF {
            gl.enable_vertex_attrib_array(vertex_attributes.uv0);
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&self.uv0.buffer),
            );
            gl.vertex_attrib_pointer_with_i32(
                vertex_attributes.uv0,
                2, // num components
                WebGl2RenderingContext::FLOAT,
                false, // normalize
                0,     // stride
                0,     // offset
            );
        }

        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.indices.buffer),
        );
    }

    /// Actually render this geometry
    pub fn render(&self, gl: &WebGl2RenderingContext) {
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices.buffer_length,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    /// Convenience function that executes bind and render. You only
    /// want this if you are only rendering a single instance of this
    /// geometry with this shader. Otherwise you can optimize by
    /// calling `bind` once and `render` lots.
    pub fn bind_and_render(
        &self,
        gl: &WebGl2RenderingContext,
        vertex_attributes: &VertexAttributes,
    ) {
        self.bind(gl, vertex_attributes);
        self.render(gl);
    }
}

impl From<JsValue> for GeometryError {
    fn from(err: JsValue) -> GeometryError {
        GeometryError::JsError(err)
    }
}

pub fn upload_f32_array(
    gl: &WebGl2RenderingContext,
    vertices: &Vec<f32>,
) -> Result<WebGlBuffer, GeometryError> {
    let position_buffer = gl
        .create_buffer()
        .ok_or(GeometryError::BufferCreationFailed)?;

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

pub fn upload_indices_array(
    gl: &WebGl2RenderingContext,
    indices: &Vec<u16>,
) -> Result<WebGlBuffer, GeometryError> {
    let index_buffer = gl
        .create_buffer()
        .ok_or(GeometryError::BufferCreationFailed)?;
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
