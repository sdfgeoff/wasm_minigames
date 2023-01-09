#version 300 es

precision mediump float;

uniform sampler2D albedo_texture;
uniform sampler2D metallic_roughness_texture;

in vec2 uv0;
in vec4 screen_pos;

in vec4 world_nor;
in float dist_from_camera;


layout(location=0) out vec4 buffer_color;
layout(location=1) out vec4 buffer_geometry;
layout(location=2) out vec4 buffer_material;

void main() {
    vec2 uv = vec2(uv0.x, 1.0 - uv0.y);

    buffer_color = texture(albedo_texture, uv);
    buffer_material = texture(metallic_roughness_texture, uv);
    buffer_geometry = vec4(world_nor.xyz, dist_from_camera);

    gl_FragDepth =  screen_pos.z / screen_pos.w;
}