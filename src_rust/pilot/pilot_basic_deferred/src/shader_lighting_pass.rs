use super::resources::Resources;
/// A shader for showing simple shapes
use super::shader;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlTexture, WebGlUniformLocation};

use super::texture::{bind_2d_texture_to_uniform, TextureUnit};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct ShaderLightingPass {
    pub program: WebGlProgram,

    pub attributes: shader::VertexAttributes,

    pub uniform_image_albedo: Option<WebGlUniformLocation>,
    pub uniform_image_normal_depth: Option<WebGlUniformLocation>,
    pub uniform_image_matcap: Option<WebGlUniformLocation>,
}

impl ShaderLightingPass {
    pub fn new(
        gl: &WebGl2RenderingContext,
        resources: &Resources,
    ) -> Result<Self, shader::ShaderError> {
        let program = shader::link_shaders(
            gl,
            &resources.vertex_shaders.full_screen_quad,
            &resources.fragment_shaders.lighting_pass,
        )?;

        let attributes = shader::VertexAttributes::new(gl, &program);

        let uniform_image_normal_depth = gl.get_uniform_location(&program, "image_normal_depth");
        let uniform_image_albedo = gl.get_uniform_location(&program, "image_albedo");
        let uniform_image_matcap = gl.get_uniform_location(&program, "image_matcap");

        Ok(Self {
            program,
            attributes,

            uniform_image_normal_depth,
            uniform_image_albedo,
            uniform_image_matcap,
        })
    }

    pub fn setup(
        &self,
        gl: &WebGl2RenderingContext,
        image_normal_depth: &Option<WebGlTexture>,
        image_albedo: &Option<WebGlTexture>,
        image_matcap: &Option<WebGlTexture>,
    ) {
        gl.use_program(Some(&self.program));

        gl.disable(WebGl2RenderingContext::BLEND);
        //gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_normal_depth,
            image_normal_depth,
            TextureUnit::Unit0,
        );
        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_albedo,
            image_albedo,
            TextureUnit::Unit1,
        );
        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_matcap,
            image_matcap,
            TextureUnit::Unit2,
        );
    }
}
