use std::convert::TryInto;
use web_sys::WebGl2RenderingContext;

use super::geometry::{Geometry, GeometryError};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn load_mesh(gl: &WebGl2RenderingContext, mesh: &[u8]) -> Result<Geometry, GeometryError> {
    let (faces, positions, normals, uv0) = extact_buffers_from_mesh(mesh);
    Geometry::new(gl, &positions, &normals, &uv0, &faces)
}

/// Reads a f32 from a buffer
fn get_f32(arr: &[u8]) -> f32 {
    f32::from_le_bytes(arr[0..4].try_into().unwrap())
}
/// Reads a u16 from a buffer
fn get_u16(arr: &[u8]) -> u16 {
    u16::from_le_bytes(arr[0..2].try_into().unwrap())
}

/// Converts a slice of u8's into a vec of f32;s
fn parse_f32_array(data: &[u8], num_elements: usize) -> Vec<f32> {
    let mut out_array = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        out_array.push(get_f32(&data[i * 4..]));
    }
    out_array
}
/// Converts a slice of u8's into a vec of u16's
fn parse_u16_array(data: &[u8], num_elements: usize) -> Vec<u16> {
    let mut out_array = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        out_array.push(get_u16(&data[i * 2..]));
    }
    out_array
}

/// Converts the bytes of a binary stl file into a vector of face indices,
/// vertices and vertex normals.
/// Expects correctly formatted STL files
fn extact_buffers_from_mesh(mesh: &[u8]) -> (Vec<u16>, Vec<f32>, Vec<f32>, Vec<f32>) {
    let num_verts = u16::from_le_bytes(mesh[0..2].try_into().unwrap()) as usize;
    let num_faces = u16::from_le_bytes(mesh[2..4].try_into().unwrap()) as usize;

    let verts_start = 4;
    let normals_start = verts_start + num_verts * 4 * 3;
    let uv0_start = normals_start + num_verts * 4 * 3;
    let indices_start = uv0_start + num_verts * 4 * 2;

    let vertices = parse_f32_array(&mesh[verts_start..], num_verts * 3);
    let normals = parse_f32_array(&mesh[normals_start..], num_verts * 3);
    let uv0 = parse_f32_array(&mesh[uv0_start..], num_verts * 2);
    let indices = parse_u16_array(&mesh[indices_start..], num_faces * 3);

    (indices, vertices, normals, uv0)
}
