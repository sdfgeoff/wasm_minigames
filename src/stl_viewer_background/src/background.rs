use std::convert::TryInto;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use super::geometry::{upload_f32_array, upload_indices_array, GeometryError};
use super::shader_background::ShaderBackground;
use glam::{Mat4, Vec3};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Background {
    position_buffer: WebGlBuffer,
}

impl Background {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, GeometryError> {
        let position_buffer =
            upload_f32_array(gl, vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0])?;

       
        Ok(Self {
            position_buffer,
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, shader_background: &ShaderBackground) {

        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        gl.vertex_attrib_pointer_with_i32(
            shader_background.attrib_vertex_positions,
            2, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );
        gl.enable_vertex_attrib_array(shader_background.attrib_vertex_positions);

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}
