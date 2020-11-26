#version 300 es

precision mediump float;
in vec2 vert_pos; // In Screen Space

out vec3 world_pos;

uniform mat4 world_to_camera;
uniform mat4 camera_to_screen;

void main() {
    
    mat4 camera_to_world = inverse(world_to_camera);
    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;
    
    vec4 world_pos_tmp = screen_to_world * vec4(vert_pos, 1.0, 1.0);
    world_pos = world_pos_tmp.xyz;// / world_pos_tmp.w;

    gl_Position.xy = vert_pos.xy;
    gl_Position.z = 0.99999;
    gl_Position.w = 1.0;
}
