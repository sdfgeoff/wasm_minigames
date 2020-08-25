use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation,
};

use super::shader::{init_shader_program, upload_array_f32, ShaderError};
use super::texture::{bind_2d_texture_to_uniform, load_texture, TextureUnit};

pub struct ShipSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_ship_engine: Option<WebGlUniformLocation>,
    uniform_ship_texture: Option<WebGlUniformLocation>,
    uniform_ship_color: Option<WebGlUniformLocation>,

    uniform_world_to_camera: Option<WebGlUniformLocation>,
    uniform_world_to_sprite: Option<WebGlUniformLocation>,
    uniform_camera_to_clipspace: Option<WebGlUniformLocation>,

    pub ship_texture: WebGlTexture,
    pub ship_color: (f32, f32, f32, f32),
    pub ship_engine: f32,

    pub world_to_camera: [f32; 9],
    pub world_to_sprite: [f32; 9],
    pub camera_to_clipspace: [f32; 9],
}

impl ShipSprite {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, ShaderError> {
        let position_buffer =
            upload_array_f32(gl, vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0])?;

        let program = init_shader_program(
            gl,
            include_str!("resources/ship.vert"),
            include_str!("resources/ship.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_ship_texture = gl.get_uniform_location(&program, "ship_texture");
        let uniform_ship_engine = gl.get_uniform_location(&program, "ship_engine");
        let uniform_ship_color = gl.get_uniform_location(&program, "ship_color");

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_sprite = gl.get_uniform_location(&program, "world_to_sprite");
        let uniform_camera_to_clipspace = gl.get_uniform_location(&program, "camera_to_clipspace");

        let ship_texture = load_texture(&gl, include_bytes!("resources/ship.png"))
            .expect("Failed to load texture");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_ship_engine,
            uniform_ship_texture,
            uniform_ship_color,

            uniform_world_to_camera,
            uniform_world_to_sprite,
            uniform_camera_to_clipspace,

            ship_color: (1.0, 1.0, 1.0, 1.0),
            ship_texture,
            ship_engine: 0.0,

            world_to_camera: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            world_to_sprite: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            camera_to_clipspace: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));

        gl.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE);

        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_world_to_sprite.as_ref(),
            true,
            &self.world_to_sprite,
        );
        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_world_to_camera.as_ref(),
            true,
            &self.world_to_camera,
        );
        gl.uniform_matrix3fv_with_f32_array(
            self.uniform_camera_to_clipspace.as_ref(),
            true,
            &self.camera_to_clipspace,
        );

        gl.uniform4f(
            self.uniform_ship_color.as_ref(),
            self.ship_color.0,
            self.ship_color.1,
            self.ship_color.2,
            self.ship_color.3,
        );
        gl.uniform1f(self.uniform_ship_engine.as_ref(), self.ship_engine);

        bind_2d_texture_to_uniform(
            &gl,
            &self.uniform_ship_texture,
            &self.ship_texture,
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

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}
