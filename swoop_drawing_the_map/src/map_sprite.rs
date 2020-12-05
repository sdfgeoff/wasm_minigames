use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::shader::{init_shader_program, upload_array_f32, ShaderError};

pub struct MapSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_world_to_camera: Option<WebGlUniformLocation>,
    uniform_world_to_sprite: Option<WebGlUniformLocation>,
    uniform_camera_to_clipspace: Option<WebGlUniformLocation>,

    pub world_to_camera: [f32; 9],
    pub world_to_sprite: [f32; 9],
    pub camera_to_clipspace: [f32; 9],
}

impl MapSprite {
    pub fn new(gl: &WebGl2RenderingContext, options: String) -> Result<Self, ShaderError> {
        let map_size = 100.0;
        let position_buffer = upload_array_f32(
            gl,
            vec![
                -map_size, map_size, map_size, map_size, -map_size, -map_size, map_size, -map_size,
            ],
        )?;

        let frag_shader = {
            match options.as_ref() {
                "coords" => include_str!("resources/map_coords.frag"),
                "circle_1" => include_str!("resources/map_circle_1.frag"),
                "circle_2" => include_str!("resources/map_circle_2.frag"),
                "fourier_1" => include_str!("resources/map_fourier_1.frag"),
                "visualized" => include_str!("resources/map_visualized.frag"),
                _ => include_str!("resources/map_visualized.frag"),
            }
        };

        let program = init_shader_program(gl, include_str!("resources/map.vert"), frag_shader)?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_sprite = gl.get_uniform_location(&program, "world_to_sprite");
        let uniform_camera_to_clipspace = gl.get_uniform_location(&program, "camera_to_clipspace");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_world_to_camera,
            uniform_world_to_sprite,
            uniform_camera_to_clipspace,

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
