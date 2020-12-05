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

pub struct ShaderStl {
    pub program: WebGlProgram,

    pub attrib_vertex_positions: u32,
    pub attrib_vertex_normals: u32,

    pub uniform_resolution: Option<WebGlUniformLocation>,
    pub uniform_time: Option<WebGlUniformLocation>,
    pub uniform_image_matcap: Option<WebGlUniformLocation>,

    pub resolution: (u32, u32),
    pub time: f32,
    pub image_matcap: Option<WebGlTexture>,
}

impl ShaderStl {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, shader::ShaderError> {
        let program = shader::init_shader_program(
            gl,
            include_str!("resources/shader.vert"),
            include_str!("resources/shader.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "vert_pos") as u32;
        let attrib_vertex_normals = gl.get_attrib_location(&program, "vert_nor") as u32;

        let uniform_resolution = gl.get_uniform_location(&program, "iResolution");
        let uniform_time = gl.get_uniform_location(&program, "iTime");
        let uniform_image_matcap = gl.get_uniform_location(&program, "image_matcap");

        Ok(Self {
            program,
            attrib_vertex_positions,
            attrib_vertex_normals,
            uniform_resolution,
            uniform_time,
            uniform_image_matcap,

            resolution: (100, 100),
            time: 0.0,
            image_matcap: None,
        })
    }

    pub fn setup(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));

        gl.uniform1f(self.uniform_time.as_ref(), self.time);
        gl.uniform2f(
            self.uniform_resolution.as_ref(),
            self.resolution.0 as f32,
            self.resolution.1 as f32,
        );

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_image_matcap,
            &self.image_matcap,
            TextureUnit::Unit0,
        );
    }
}
