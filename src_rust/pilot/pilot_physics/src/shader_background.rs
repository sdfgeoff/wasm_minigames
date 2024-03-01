use super::resources::Resources;
/// A shader for showing simple shapes
use super::shader;

use glam::Mat4;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlTexture, WebGlUniformLocation};

use super::texture::{bind_2d_texture_to_uniform, TextureUnit};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct ShaderBackground {
    pub program: WebGlProgram,

    pub attributes: shader::VertexAttributes,

    pub uniform_image_world_texture: Option<WebGlUniformLocation>,

    pub uniform_world_to_camera: Option<WebGlUniformLocation>,
    pub uniform_world_to_model: Option<WebGlUniformLocation>,
    pub uniform_camera_to_screen: Option<WebGlUniformLocation>,

    pub image_world_texture: Option<WebGlTexture>,
}

impl ShaderBackground {
    pub fn new(
        gl: &WebGl2RenderingContext,
        resources: &Resources,
    ) -> Result<Self, shader::ShaderError> {
        let program = shader::link_shaders(
            gl,
            &resources.vertex_shaders.background,
            &resources.fragment_shaders.background,
        )?;

        let attributes = shader::VertexAttributes::new(gl, &program);

        let uniform_image_world_texture = gl.get_uniform_location(&program, "image_world_texture");

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_model = gl.get_uniform_location(&program, "world_to_model");
        let uniform_camera_to_screen = gl.get_uniform_location(&program, "camera_to_screen");

        Ok(Self {
            program,
            attributes,

            uniform_image_world_texture,

            uniform_world_to_camera,
            uniform_world_to_model,
            uniform_camera_to_screen,

            image_world_texture: None,
        })
    }

    pub fn setup(
        &self,
        gl: &WebGl2RenderingContext,
        world_to_camera: Mat4,
        camera_to_screen: Mat4,
    ) {
        gl.use_program(Some(&self.program));

        gl.uniform_matrix4fv_with_f32_array(
            self.uniform_world_to_camera.as_ref(),
            false,
            &world_to_camera.to_cols_array(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.uniform_camera_to_screen.as_ref(),
            false,
            &camera_to_screen.to_cols_array(),
        );

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_world_texture,
            &self.image_world_texture,
            TextureUnit::Unit0,
        );
    }
}
