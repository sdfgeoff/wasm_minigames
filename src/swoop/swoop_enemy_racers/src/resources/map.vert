#version 300 es

precision mediump float;
in vec4 aVertexPosition;

uniform mat3 world_to_camera;
uniform mat3 world_to_sprite;
uniform mat3 camera_to_clipspace; // Includes canvas resolution/aspect ratio

out vec2 uv;

void main() {
	mat3 camera_to_world = inverse(world_to_camera);
	mat3 clipspace_to_camera = inverse(camera_to_clipspace);
	mat3 camera_to_sprite = camera_to_world * world_to_sprite;
	mat3 sprite_to_clipspace = clipspace_to_camera * camera_to_sprite;
	
	vec2 pos = (sprite_to_clipspace * vec3(aVertexPosition.xy, 1.0)).xy;
	
	uv = aVertexPosition.xy;
	gl_Position = vec4(pos, 0.0, 1.0);
}
