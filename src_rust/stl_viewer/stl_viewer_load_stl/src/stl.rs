use std::convert::TryInto;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use super::geometry::{upload_f32_array, upload_indices_array, GeometryError};
use super::shader_stl::ShaderStl;

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Stl {
    position_buffer: WebGlBuffer,
    normal_buffer: WebGlBuffer,
    faces_buffer: WebGlBuffer,
    num_face_indices: u16,
}

impl Stl {
    pub fn new(gl: &WebGl2RenderingContext, stl: &[u8]) -> Result<Self, GeometryError> {
        let (faces, positions, normals) = extact_buffers_from_stl(stl);

        let position_buffer = upload_f32_array(gl, positions)?;
        let normal_buffer = upload_f32_array(gl, normals)?;
        let num_face_indices = faces.len() as u16;
        let faces_buffer = upload_indices_array(gl, faces)?;

        Ok(Self {
            position_buffer,
            normal_buffer,
            faces_buffer,
            num_face_indices,
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, shader_stl: &ShaderStl) {
        gl.enable_vertex_attrib_array(shader_stl.attrib_vertex_positions);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            shader_stl.attrib_vertex_positions,
            3, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );

        gl.enable_vertex_attrib_array(shader_stl.attrib_vertex_normals);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.normal_buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            shader_stl.attrib_vertex_normals,
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

/// Reads a f32 from a buffer
fn get_f32(arr: &[u8], offset: u32) -> f32 {
    let offset = offset as usize;
    f32::from_le_bytes(arr[offset..offset + 4].try_into().unwrap())
}

/// Converts the bytes of a binary stl file into a vector of face indices,
/// vertices and vertex normals.
/// Expects correctly formatted STL files
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
