#version 300 es

precision highp float;
in vec3 attribute_vertex_position;
in vec3 attribute_vertex_normal;
in vec2 attribute_vertex_uv0;

uniform mat4 world_to_model;
uniform mat4 model_to_world;
uniform mat4 camera_to_screen;
uniform mat4 camera_to_world;
uniform mat4 world_to_camera;


out vec4 screen_nor;
out vec4 screen_pos;
out vec4 world_nor;
out vec4 world_pos;

out vec2 uv0;


void main() {
	mat4 model_to_camera = world_to_camera * model_to_world;
    mat4 model_to_screen = camera_to_screen * model_to_camera;
    
    vec4 pos = vec4(attribute_vertex_position, 1.0);
    vec4 nor = vec4(attribute_vertex_normal, 0.0);

    uv0 = attribute_vertex_uv0;

    screen_pos = model_to_screen * pos;

    screen_nor = model_to_screen * nor;
    world_nor = model_to_world * nor;
    world_pos = model_to_world * pos;

    gl_Position = screen_pos;
}