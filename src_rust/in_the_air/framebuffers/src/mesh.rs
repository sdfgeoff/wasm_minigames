use super::mesh_loader;
/// LOOK AT:
/// https://rust-tutorials.github.io/learn-opengl/basics/001-drawing-a-triangle.html
use glow::{Buffer, Context, HasContext, ARRAY_BUFFER, FLOAT, STATIC_DRAW};

/// An error with this whole object.
#[derive(Debug)]
pub enum MeshError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed(String),
}

pub struct Mesh {
    position_buffer: Buffer,
    normal_buffer: Buffer,
    uv0_buffer: Buffer,
    indices_buffer: Buffer,
    num_indices: i32,
}

impl Mesh {
    pub fn bind(&self, gl: &Context, attrib_vertex_positions: u32) {
        unsafe {
            gl.enable_vertex_attrib_array(attrib_vertex_positions);
            gl.bind_buffer(ARRAY_BUFFER, Some(self.position_buffer));

            gl.vertex_attrib_pointer_f32(
                attrib_vertex_positions, 
                3,                       
                FLOAT,               
                false,            
                0,                    
                0,                    
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.indices_buffer));
        }
    }

    /// Actually render this geometry
    pub fn render(&self, gl: &Context) {
        unsafe {
            gl.draw_elements(glow::TRIANGLES, self.num_indices, glow::UNSIGNED_SHORT, 0);
        }
    }

    pub fn load_from_bytes(gl: &Context, mesh: &[u8]) -> Result<Self, MeshError> {
        let (indices, positions, normals, uv0) = mesh_loader::extract_buffers_from_mesh(mesh);
        Ok(Self {
            position_buffer: unsafe { upload_array_f32(gl, &positions)? },
            normal_buffer: unsafe { upload_array_f32(gl, &normals)? },
            uv0_buffer: unsafe { upload_array_f32(gl, &uv0)? },
            indices_buffer: unsafe { upload_indices_array(gl, &indices)? },
            num_indices: indices.len() as i32,
        })
    }
}

unsafe fn upload_array_f32(gl: &Context, vertices: &[f32]) -> Result<Buffer, MeshError> {
    let vao = gl
        .create_vertex_array()
        .map_err(MeshError::BufferCreationFailed)?;
    gl.bind_vertex_array(Some(vao));
    let vbo = gl
        .create_buffer()
        .map_err(MeshError::BufferCreationFailed)?;
    gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

    gl.buffer_data_u8_slice(ARRAY_BUFFER, vec_f32_as_u8_slice(&vertices), STATIC_DRAW);

    Ok(vbo)
}

unsafe fn upload_indices_array(gl: &Context, indices: &[u16]) -> Result<Buffer, MeshError> {
    let index_buffer = gl
        .create_buffer()
        .map_err(MeshError::BufferCreationFailed)?;
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));

    gl.buffer_data_u8_slice(
        glow::ELEMENT_ARRAY_BUFFER,
        vec_u16_as_u8_slice(&indices),
        glow::STATIC_DRAW,
    );

    Ok(index_buffer)
}

fn vec_f32_as_u8_slice(v: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

fn vec_u16_as_u8_slice(v: &[u16]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}
