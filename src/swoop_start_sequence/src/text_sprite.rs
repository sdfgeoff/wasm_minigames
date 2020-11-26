use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation,
};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use super::shader::{init_shader_program, upload_array_f32, ShaderError};
use super::texture::{bind_2d_texture_to_uniform, load_texture, TextureUnit};


pub struct TextSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_font_texture: Option<WebGlUniformLocation>,
    pub font_texture: WebGlTexture,

    uniform_text_data: Option<WebGlUniformLocation>,
    uniform_box_dimensions: Option<WebGlUniformLocation>,
    uniform_character_height: Option<WebGlUniformLocation>,
    uniform_screen_aspect: Option<WebGlUniformLocation>,
    uniform_anchor: Option<WebGlUniformLocation>,
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
        let uniform_box_dimensions = gl.get_uniform_location(&program, "text_box_dimensions");
        let uniform_character_height = gl.get_uniform_location(&program, "character_height");
        let uniform_anchor = gl.get_uniform_location(&program, "anchor");
        let uniform_screen_aspect = gl.get_uniform_location(&program, "screen_aspect");
        let uniform_text_data = gl.get_uniform_location(&program, "text_data");
        
        let font_texture = load_texture(&gl, include_bytes!("resources/font.png"))
            .expect("Failed to load texture");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_font_texture,
            font_texture,

            uniform_box_dimensions,
            uniform_character_height,
            uniform_anchor,
            uniform_text_data,
            uniform_screen_aspect
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

    pub fn render(&mut self, gl: &WebGl2RenderingContext, text_box: &TextBox, screen_aspect: f32,) {
        // gl.uniform1f(
        //     self.uniform_trail_percent_offset.as_ref(),
        //     trail.get_percent_offset(),
        // );

        let text_data = text_box.uniform_data();
        //gl.uniform1i(self.uniform_point_buffer_length.as_ref(), trail.length());
        gl.uniform4fv_with_f32_array(self.uniform_text_data.as_ref(), &text_data);
        gl.uniform2i(self.uniform_box_dimensions.as_ref(), text_box.box_dimensions.0, text_box.box_dimensions.1);
        
        gl.uniform1f(self.uniform_character_height.as_ref(), text_box.character_height);
        gl.uniform2f(self.uniform_anchor.as_ref(), text_box.anchor.0, text_box.anchor.1);

        gl.uniform1f(self.uniform_screen_aspect.as_ref(), screen_aspect);

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}

/// The thing that can be drawn with a text sprite.
pub struct TextBox {
    data: Vec<f32>,

    // The text box wraps character wise and permits width*height characters to be displayed
    box_dimensions: (i32, i32),

    /// Height of a single character As percentage of screen size
    character_height: f32, 
    
    /// Where on the screen to draw the text. Positions the center of the text box with the screen ranging from -1.0 to 1.0 on both axis.
    anchor: (f32, f32)
}

impl TextBox {
    const VALID_CHARS: &'static str = "0123456789ABCDFEGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz:-<>*[] ";
    
    pub fn new(box_dimensions: (i32, i32), character_height: f32, anchor: (f32, f32)) -> Self {
        Self {
            data: vec!(),
            box_dimensions,
            character_height,
            anchor,
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    pub fn append_string(&mut self, string: &str, color: &[f32; 3]){
        for c in string.chars() {
            self.data.extend(color);
            self.data.push(Self::encode_char(c));
        }
    }

    /// The Text sprite has characters encoded in a non-standard order.
    /// This does the conversion
    fn encode_char(c: char) -> f32 {
        match Self::VALID_CHARS.find(c) {
            Some(id) => {
                id as f32
            }
            None => {
                -1.0
            }
        }
    }
    // Retrieve the data in a format that can be posted to teh shader
    pub fn uniform_data(&self) -> &Vec<f32> {
        &self.data
    }
}
