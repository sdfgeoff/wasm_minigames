use crate::mesh::{Mesh, MeshError};
use crate::shader::{Shader, ShaderError, ShaderType};
use crate::texture::{Texture, TextureConfig, TextureError};

use glow::Context;

pub struct Textures {
    pub vehicle_roughness_metal: Texture,
    pub vehicle_albedo: Texture,
}

impl Textures {
    pub fn load(gl: &Context) -> Result<Self, TextureError> {
        Ok(Self {
            vehicle_roughness_metal: Texture::load_from_png(
                gl,
                include_bytes!("vehicle_roughness_metal.png"),
                TextureConfig::default(),
            )?,
            vehicle_albedo: Texture::load_from_png(
                gl,
                include_bytes!("vehicle_albedo.png"),
                TextureConfig::default(),
            )?,
        })
    }
}

pub struct Meshes {
    pub vehicle_med_res: Mesh,
    pub quad_quad: Mesh,
}

impl Meshes {
    pub fn load(gl: &Context) -> Result<Self, MeshError> {
        Ok(Self {
            vehicle_med_res: Mesh::load_from_bytes(gl, include_bytes!("vehicle_med_res.mesh"))?,
            quad_quad: Mesh::load_from_bytes(gl, include_bytes!("quad_quad.mesh"))?,
        })
    }
}

pub struct VertexShaders {
    pub full_screen_quad: Shader,
    pub model_shader: Shader,
}
impl VertexShaders {
    pub fn load(gl: &Context) -> Result<Self, ShaderError> {
        Ok(Self {
            full_screen_quad: Shader::new(
                gl,
                ShaderType::Vertex,
                include_str!("full_screen_quad.vert"),
            )?,
            model_shader: Shader::new(gl, ShaderType::Vertex, include_str!("model_shader.vert"))?,
        })
    }
}

pub struct FragmentShaders {
    pub model_shader: Shader,
    pub volume_and_light: Shader,
    pub passthrough: Shader,
}
impl FragmentShaders {
    pub fn load(gl: &Context) -> Result<Self, ShaderError> {
        Ok(Self {
            model_shader: Shader::new(gl, ShaderType::Fragment, include_str!("model_shader.frag"))?,
            volume_and_light: Shader::new(
                gl,
                ShaderType::Fragment,
                include_str!("volume_and_light.frag"),
            )?,
            passthrough: Shader::new(gl, ShaderType::Fragment, include_str!("passthrough.frag"))?,
        })
    }
}

pub struct StaticResources {
    pub textures: Textures,
    pub meshes: Meshes,
    pub fragment_shaders: FragmentShaders,
    pub vertex_shaders: VertexShaders,
}

impl StaticResources {
    pub fn load(gl: &Context) -> Self {
        Self {
            textures: Textures::load(gl).expect("Failed loading textures"),
            meshes: Meshes::load(gl).expect("Failed loading meshes"),
            fragment_shaders: FragmentShaders::load(gl).expect("Failed loading fragment shaders"),
            vertex_shaders: VertexShaders::load(gl).expect("Failed loading vertex shaders"),
        }
    }
}
