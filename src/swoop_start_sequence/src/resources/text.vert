#version 300 es

precision highp float;
precision highp int;
in vec4 aVertexPosition;

uniform mat3 world_to_camera;
uniform mat3 world_to_sprite;
uniform mat3 camera_to_clipspace; // Includes canvas resolution/aspect ratio

out vec2 uv;

uniform ivec2 text_box_dimensions;
uniform float character_height;
uniform vec2 anchor;
uniform float screen_aspect;


void main() {
	//~ mat3 camera_to_world = inverse(world_to_camera);
	//~ mat3 clipspace_to_camera = inverse(camera_to_clipspace);
	//~ mat3 camera_to_sprite = camera_to_world * world_to_sprite;
	//~ mat3 sprite_to_clipspace = clipspace_to_camera * camera_to_sprite;
	
	//~ vec2 pos = (sprite_to_clipspace * vec3(aVertexPosition.xy, 1.0)).xy;
	
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
