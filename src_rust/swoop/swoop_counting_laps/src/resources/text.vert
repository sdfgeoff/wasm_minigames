#version 300 es

precision highp float;
precision highp int;
in vec4 aVertexPosition;

uniform mat3 world_to_camera;
uniform mat3 world_to_sprite;
uniform mat3 camera_to_clipspace; // Includes canvas resolution/aspect ratio

out vec2 uv;

/// How many characters wide and high the text box is
uniform ivec2 text_box_dimensions;

/// how tall (in screen space) a single character should be
uniform float character_height;

/// Where the center of the text box should be located (in screen space)
uniform vec2 anchor;

/// Aspect ratio of the screen
uniform float screen_aspect;


void main() {
	float character_width = character_height * 5.0 / 9.0;
	vec2 text_box_size = vec2(
		character_width * float(text_box_dimensions.x),
		character_height * float(text_box_dimensions.y)
	);

	uv = aVertexPosition.xy;
	vec2 pos = uv * text_box_size + anchor;
	pos.x *= screen_aspect;
	gl_Position = vec4(pos, 0.0, 1.0);
}
