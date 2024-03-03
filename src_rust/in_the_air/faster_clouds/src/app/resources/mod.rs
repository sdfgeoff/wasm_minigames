use crate::app::mesh::{Mesh, MeshError};
use crate::app::shader::{Shader, ShaderError, ShaderType};
use crate::app::texture::{
    Dimension, EdgeMode, InterpolationMode, Texture, TextureConfig, TextureError,
};

use glow::Context;

pub struct Textures {
    pub vehicle_roughness_metal: Texture,
    pub vehicle_albedo: Texture,
    pub test_tex: Texture,
    pub cloud_map: Texture,
    pub volume_noise: Texture,
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
            test_tex: Texture::load_from_png(
                gl,
                include_bytes!("test_tex.png"),
                TextureConfig::default(),
            )?,
            cloud_map: Texture::load_from_png(
                gl,
                include_bytes!("cloud_map.png"),
                TextureConfig::default(),
            )?,
            volume_noise: Texture::load_from_png(
                gl,
                include_bytes!("volume_noise.png"),
                TextureConfig {
                    generate_mipmap: false,
                    mag_interpolation: InterpolationMode::Linear,
                    min_interpolation: InterpolationMode::Linear,
                    edge_behaviour: EdgeMode::Repeat,
                    dimension: Dimension::D3(32, 32, 32),
                },
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
    pub volume: Shader,
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
            volume: Shader::new(gl, ShaderType::Fragment, include_str!("volume.frag"))?,
            passthrough: Shader::new(gl, ShaderType::Fragment, include_str!("passthrough.frag"))?,
        })
    }
}

#[derive(Debug)]
pub enum ResourceError {
    TextureError(TextureError),
    MeshError(MeshError),
    ShaderError(ShaderError),
}

pub struct StaticResources {
    pub textures: Textures,
    pub meshes: Meshes,
    pub fragment_shaders: FragmentShaders,
    pub vertex_shaders: VertexShaders,
}

impl StaticResources {
    pub fn load(gl: &Context) -> Result<Self, ResourceError> {
        Ok(Self {
            textures: Textures::load(gl).map_err(ResourceError::TextureError)?,
            meshes: Meshes::load(gl).map_err(ResourceError::MeshError)?,
            fragment_shaders: FragmentShaders::load(gl).map_err(ResourceError::ShaderError)?,
            vertex_shaders: VertexShaders::load(gl).map_err(ResourceError::ShaderError)?,
        })
    }
}
