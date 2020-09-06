use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::engine_trail::EngineTrail;
use super::shader::{init_shader_program, upload_array_f32, ShaderError};

const SEGMENT_COUNT: i32 = 100;

pub struct EngineTrailSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_world_to_camera: Option<WebGlUniformLocation>,
    uniform_world_to_sprite: Option<WebGlUniformLocation>,
    uniform_camera_to_clipspace: Option<WebGlUniformLocation>,

    uniform_point_buffer: Option<WebGlUniformLocation>,
    uniform_data_buffer: Option<WebGlUniformLocation>,
    uniform_point_buffer_length: Option<WebGlUniformLocation>,
    uniform_trail_color: Option<WebGlUniformLocation>,
    uniform_trail_percent_offset: Option<WebGlUniformLocation>,

    pub world_to_camera: [f32; 9],
    pub world_to_sprite: [f32; 9],
    pub camera_to_clipspace: [f32; 9],
}

impl EngineTrailSprite {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, ShaderError> {
        let mut position_buffer = vec![-1.0, 0.0];
        position_buffer.extend(&vec![1.0, 0.0]);

        for i in 1..SEGMENT_COUNT + 1 {
            position_buffer.extend(&vec![-1.0, i as f32 / SEGMENT_COUNT as f32]);
            position_buffer.extend(&vec![1.0, i as f32 / SEGMENT_COUNT as f32]);
        }

        let position_buffer = upload_array_f32(gl, position_buffer)?;

        let program = init_shader_program(
            gl,
            include_str!("resources/trail.vert"),
            include_str!("resources/trail.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_point_buffer = gl.get_uniform_location(&program, "point_buffer");
        let uniform_data_buffer = gl.get_uniform_location(&program, "data_buffer");
        let uniform_point_buffer_length = gl.get_uniform_location(&program, "point_buffer_length");
        let uniform_trail_color = gl.get_uniform_location(&program, "trail_color");
        let uniform_trail_percent_offset =
            gl.get_uniform_location(&program, "trail_percent_offset");

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

            uniform_point_buffer,
            uniform_data_buffer,
            uniform_point_buffer_length,
            uniform_trail_color,
            uniform_trail_percent_offset,

            world_to_camera: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            world_to_sprite: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            camera_to_clipspace: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, trail: &EngineTrail) {
        gl.use_program(Some(&self.program));

        gl.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE);

        gl.uniform4f(
            self.uniform_trail_color.as_ref(),
            trail.color.0,
            trail.color.1,
            trail.color.2,
            trail.color.3,
        );

        gl.uniform1f(
            self.uniform_trail_percent_offset.as_ref(),
            trail.get_percent_offset(),
        );

        let (point_buffer, data_buffer) = trail.path_data_buffers();
        gl.uniform1i(self.uniform_point_buffer_length.as_ref(), trail.length());
        gl.uniform4fv_with_f32_array(self.uniform_point_buffer.as_ref(), &point_buffer);
        gl.uniform4fv_with_f32_array(self.uniform_data_buffer.as_ref(), &data_buffer);

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
            0,                     //offset,
            SEGMENT_COUNT * 2 + 2, // vertex count
        );
    }
}
