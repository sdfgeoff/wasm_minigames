use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

/// An error with this whole object.
#[derive(Debug)]
pub enum GeometryError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed,

    /// An unhandled/unspecified error
    JsError(JsValue),
}

impl From<JsValue> for GeometryError {
    fn from(err: JsValue) -> GeometryError {
        GeometryError::JsError(err)
    }
}

pub fn upload_f32_array(
    gl: &WebGl2RenderingContext,
    vertices: Vec<f32>,
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
    indices: Vec<u16>,
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
