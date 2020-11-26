use std::convert::TryInto;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use super::geometry::{upload_f32_array, upload_indices_array, GeometryError};
use super::shader_stl::ShaderStl;
use glam::{Mat4, Vec3};

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
    pub world_to_model: Mat4,
}

impl Stl {
    pub fn new(gl: &WebGl2RenderingContext, stl: &[u8]) -> Result<Self, GeometryError> {
        let (faces, positions, normals) = extact_buffers_from_stl(stl, false);

        let position_buffer = upload_f32_array(gl, positions)?;
        let normal_buffer = upload_f32_array(gl, normals)?;
        let num_face_indices = faces.len() as u16;
        let faces_buffer = upload_indices_array(gl, faces)?;

        Ok(Self {
            position_buffer,
            normal_buffer,
            faces_buffer,
            num_face_indices,
            world_to_model: Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0))
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, shader_stl: &ShaderStl) {
        gl.uniform_matrix4fv_with_f32_array(
            shader_stl.uniform_world_to_model.as_ref(),
            false,
            &self.world_to_model.to_cols_array(),
        );
        
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
fn extact_buffers_from_stl(stl: &[u8], generate_normals: bool) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    let mut faces = std::vec::Vec::new();
    let mut vertices = std::vec::Vec::new();
    let mut normals = std::vec::Vec::new();

    let num_faces = u32::from_le_bytes(stl[80..84].try_into().unwrap());

    for face in 0..(num_faces) {
        const STRIDE: u32 = 4 * 12 + 2;
        const OFFSET: u32 = 84;

        let face_offset = OFFSET + STRIDE * face;
        
        
        let v1 = Vec3::new(
            get_f32(stl, face_offset + 4 * 3),
            get_f32(stl, face_offset + 4 * 4),
            get_f32(stl, face_offset + 4 * 5),
        );
        
        let v2 = Vec3::new(
            get_f32(stl, face_offset + 4 * 6),
            get_f32(stl, face_offset + 4 * 7),
            get_f32(stl, face_offset + 4 * 8),
        );
        
        let v3 = Vec3::new(
            get_f32(stl, face_offset + 4 * 9),
            get_f32(stl, face_offset + 4 * 10),
            get_f32(stl, face_offset + 4 * 11),
        );
        
        let face_normal = {
            if generate_normals {
                (v1 - v2).cross(v1 - v3).normalize()
            } else {
                Vec3::new(
                    get_f32(stl, face_offset + 4 * 0),
                    get_f32(stl, face_offset + 4 * 1),
                    get_f32(stl, face_offset + 4 * 2),
                )
            }
        };
        
        
        vertices.extend(v1.as_ref());
        vertices.extend(v2.as_ref());
        vertices.extend(v3.as_ref());
        
        normals.extend(face_normal.as_ref());
        normals.extend(face_normal.as_ref());
        normals.extend(face_normal.as_ref());

        faces.push((face * 3) as u16);
        faces.push((face * 3 + 1) as u16);
        faces.push((face * 3 + 2) as u16);
    }

    (faces, vertices, normals)
}
