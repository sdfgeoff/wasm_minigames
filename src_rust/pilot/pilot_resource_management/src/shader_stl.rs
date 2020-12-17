use super::resources::Resources;
/// A shader for showing simple shapes
use super::shader;

use glam::{Mat4, Vec4};

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlTexture, WebGlUniformLocation};

use super::texture::{bind_2d_texture_to_uniform, TextureUnit};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct ShaderStl {
    pub program: WebGlProgram,

    pub attributes: shader::VertexAttributes,

    pub uniform_image_matcap: Option<WebGlUniformLocation>,
    pub uniform_color: Option<WebGlUniformLocation>,

    pub uniform_world_to_camera: Option<WebGlUniformLocation>,
    pub uniform_world_to_model: Option<WebGlUniformLocation>,
    pub uniform_camera_to_screen: Option<WebGlUniformLocation>,

    pub image_matcap: Option<WebGlTexture>,
}

impl ShaderStl {
    pub fn new(
        gl: &WebGl2RenderingContext,
        resources: &Resources,
    ) -> Result<Self, shader::ShaderError> {
        let program = shader::link_shaders(
            gl,
            &resources.vertex_shaders.model,
            &resources.fragment_shaders.model,
        )?;

        let attributes = shader::VertexAttributes::new(gl, &program);

        let uniform_image_matcap = gl.get_uniform_location(&program, "image_matcap");
        let uniform_color = gl.get_uniform_location(&program, "color");

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_model = gl.get_uniform_location(&program, "world_to_model");
        let uniform_camera_to_screen = gl.get_uniform_location(&program, "camera_to_screen");

        Ok(Self {
            program,
            attributes,

            uniform_image_matcap,
            uniform_color,

            uniform_world_to_camera,
            uniform_world_to_model,
            uniform_camera_to_screen,

            image_matcap: None,
        })
    }

    pub fn setup(
        &self,
        gl: &WebGl2RenderingContext,
        world_to_camera: Mat4,
        camera_to_screen: Mat4,
    ) {
        gl.use_program(Some(&self.program));
        
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

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
            &self.uniform_image_matcap,
            &self.image_matcap,
            TextureUnit::Unit0,
        );
    }

    pub fn set_entity_data(&self, gl: &WebGl2RenderingContext, world_to_model: Mat4, color: Vec4) {
        gl.uniform_matrix4fv_with_f32_array(
            self.uniform_world_to_model.as_ref(),
            false,
            &world_to_model.to_cols_array(),
        );

        gl.uniform4fv_with_f32_array(self.uniform_color.as_ref(), color.as_ref());
    }
}
