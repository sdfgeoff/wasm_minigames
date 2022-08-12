#version 300 es

precision mediump float;
in vec3 screen_nor;
in vec3 world_nor;
in vec3 screen_pos;
in vec2 uv0;

layout(location=0) out vec4 normal_depth;
layout(location=1) out vec4 albedo;

in mat4 camera_to_world;

uniform sampler2D image_albedo;
uniform vec4 color;

void main() {
    albedo = color * texture(image_albedo, uv0);
    normal_depth = vec4(world_nor, 1.0);
}

