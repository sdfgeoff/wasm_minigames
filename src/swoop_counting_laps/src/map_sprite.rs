use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::map::Map;
use super::shader::{init_shader_program, upload_array_f32, ShaderError};

pub struct MapSprite {
    position_buffer: WebGlBuffer,
    program: WebGlProgram,
    attrib_vertex_positions: u32,

    uniform_world_to_camera: Option<WebGlUniformLocation>,
    uniform_world_to_sprite: Option<WebGlUniformLocation>,
    uniform_camera_to_clipspace: Option<WebGlUniformLocation>,

    uniform_sin_consts: Option<WebGlUniformLocation>,
    uniform_cos_consts: Option<WebGlUniformLocation>,
    uniform_track_base_radius: Option<WebGlUniformLocation>,
    uniform_track_width: Option<WebGlUniformLocation>,

    uniform_start_line_position: Option<WebGlUniformLocation>,
    uniform_start_line_tangent: Option<WebGlUniformLocation>,

    pub world_to_camera: [f32; 9],
    pub world_to_sprite: [f32; 9],
    pub camera_to_clipspace: [f32; 9],
}

impl MapSprite {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, ShaderError> {
        let map_size = 100.0;
        let position_buffer = upload_array_f32(
            gl,
            vec![
                -map_size, map_size, map_size, map_size, -map_size, -map_size, map_size, -map_size,
            ],
        )?;

        let program = init_shader_program(
            gl,
            include_str!("resources/map.vert"),
            include_str!("resources/map.frag"),
        )?;

        let attrib_vertex_positions = gl.get_attrib_location(&program, "aVertexPosition") as u32;

        let uniform_world_to_camera = gl.get_uniform_location(&program, "world_to_camera");
        let uniform_world_to_sprite = gl.get_uniform_location(&program, "world_to_sprite");
        let uniform_camera_to_clipspace = gl.get_uniform_location(&program, "camera_to_clipspace");

        let uniform_sin_consts = gl.get_uniform_location(&program, "sin_consts");
        let uniform_cos_consts = gl.get_uniform_location(&program, "cos_consts");
        let uniform_track_base_radius = gl.get_uniform_location(&program, "track_base_radius");
        let uniform_track_width = gl.get_uniform_location(&program, "track_width");
        let uniform_start_line_tangent = gl.get_uniform_location(&program, "start_line_tangent");
        let uniform_start_line_position = gl.get_uniform_location(&program, "start_line_position");

        Ok(Self {
            position_buffer,
            program,
            attrib_vertex_positions,

            uniform_world_to_camera,
            uniform_world_to_sprite,
            uniform_camera_to_clipspace,

            uniform_sin_consts,
            uniform_cos_consts,
            uniform_track_base_radius,
            uniform_track_width,
            uniform_start_line_tangent,
            uniform_start_line_position,

            world_to_camera: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            world_to_sprite: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            camera_to_clipspace: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        })
    }

    /// Configure the shader for a specific map object
    pub fn set_to_map(&mut self, gl: &WebGl2RenderingContext, map: &Map) {
        gl.use_program(Some(&self.program));

        gl.uniform4fv_with_f32_array(self.uniform_sin_consts.as_ref(), &map.sin_consts);
        gl.uniform4fv_with_f32_array(self.uniform_cos_consts.as_ref(), &map.cos_consts);
        gl.uniform1f(
            self.uniform_track_base_radius.as_ref(),
            map.track_base_radius,
        );
        gl.uniform1f(self.uniform_track_width.as_ref(), map.track_width);

        let start_position = map.get_start_position();
        let start_angle = map.get_track_direction(start_position.angle);
        let start_tangent = (f32::cos(start_angle), f32::sin(start_angle));

        let start_position_cartesian = start_position.to_cartesian();

        gl.uniform2f(
            self.uniform_start_line_position.as_ref(),
            start_position_cartesian.0,
            start_position_cartesian.1,
        );
        gl.uniform2f(
            self.uniform_start_line_tangent.as_ref(),
            start_tangent.0,
            start_tangent.1,
        );
    }

    /// Render the map sprite.
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
