#version 300 es

precision mediump float;
in vec3 vert_pos;
in vec3 vert_nor;
in vec2 vert_uv0;

out vec3 screen_nor;
out vec3 screen_pos;
out vec3 world_nor;
out vec2 uv0;

out mat4 camera_to_world;


uniform mat4 world_to_camera;
uniform mat4 world_to_model;
uniform mat4 camera_to_screen;

void main() {
    mat4 model_to_world = inverse(world_to_model);
    mat4 model_to_camera = world_to_camera * model_to_world;
    mat4 model_to_screen = camera_to_screen * model_to_camera;
    
    vec4 pos = vec4(vert_pos, 1.0);
    vec4 nor = vec4(vert_nor, 0.0);
    
    world_nor = (model_to_world * nor).xyz;
    
    camera_to_world = inverse(world_to_camera);

    pos = model_to_screen * pos;
    nor = model_to_camera * nor;    
    
    uv0 = vert_uv0;    
    
    screen_pos = pos.xyz / pos.w;
    screen_nor = nor.xyz;
    
    gl_Position = pos;

}
