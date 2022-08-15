use super::mesh::{Mesh, MeshError};
use super::shader::{Shader, ShaderError};
use glow::{Context, HasContext};

pub struct Meshes {
    quad: Mesh,
}

pub struct Shaders {
    test_shader: Shader,
}

pub struct RendererState {
    pub resolution: (i32, i32),
    pub pixels_per_centimeter: f64,
    pub meshes: Meshes,
    pub shaders: Shaders,
}

pub struct WorldState {
    pub time: f32,
}

pub fn load_meshes(gl: &Context) -> Result<Meshes, MeshError> {
    let quad = Mesh::new(
        gl,
        &[-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
        &[0, 1, 2, 0, 2, 3],
    )?;

    Ok(Meshes { quad })
}

pub fn load_shaders(gl: &Context) -> Result<Shaders, ShaderError> {
    let test_shader = Shader::new(
        gl,
        include_str!("resources/test.vert"),
        include_str!("resources/test.frag"),
    )?;

    Ok(Shaders { test_shader })
}


pub fn render(gl: &Context, renderer_state: &RendererState, world_state: &WorldState) {
    unsafe {
        gl.viewport(0, 0, renderer_state.resolution.0, renderer_state.resolution.1);

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        renderer_state.shaders.test_shader.bind(gl);
        renderer_state.meshes.quad.bind(gl, renderer_state.shaders.test_shader.attrib_vertex_positions);
        renderer_state.meshes.quad.render(gl);
    }
}