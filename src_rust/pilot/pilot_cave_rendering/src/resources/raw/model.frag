#version 300 es

precision mediump float;
in vec4 screen_nor;
in vec4 world_nor;
in vec4 screen_pos;
in vec2 uv0;
in float dist_from_camera;

layout(location=0) out vec4 normal_depth;
layout(location=1) out vec4 albedo;

in mat4 camera_to_world;

uniform sampler2D image_albedo;
uniform vec4 color;

void main() {
    albedo = color * texture(image_albedo, uv0);
    normal_depth = vec4(world_nor.xyz, dist_from_camera);
    
    gl_FragDepth =  screen_pos.z / screen_pos.w;
}

