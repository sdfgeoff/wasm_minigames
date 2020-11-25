use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation,
};

use super::shader::{init_shader_program, upload_array_f32, ShaderError};
use super::texture::{bind_2d_texture_to_uniform, load_texture, TextureUnit};


pub struct TextSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_font_texture: Option<WebGlUniformLocation>,
    pub font_texture: WebGlTexture,
}

impl TextSprite {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, ShaderError> {
        let position_buffer =
            upload_array_f32(gl, vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0])?;

        let program = init_shader_program(
            gl,
            include_str!("resources/text.vert"),
            include_str!("resources/text.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_font_texture = gl.get_uniform_location(&program, "font_texture");
        
        let font_texture = load_texture(&gl, include_bytes!("resources/font.png"))
            .expect("Failed to load texture");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_font_texture,
            
            font_texture,
        })
    }

    pub fn setup(&mut self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));
        gl.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE);

        

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_font_texture,
            &self.font_texture,
            TextureUnit::Unit0,
        );

        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        gl.vertex_attrib_pointer_with_i32(
            self.attrib_vertex_positions,
            2, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );
        gl.enable_vertex_attrib_array(self.attrib_vertex_positions);
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}
