#version 300 es

precision mediump float;

uniform sampler2D albedo_texture;
uniform sampler2D metallic_roughness_texture;

in vec2 uv0;



layout(location=0) out vec4 buffer_color;
layout(location=1) out vec4 buffer_geometry;
layout(location=2) out vec4 buffer_material;

void main() {
    buffer_color = texture(albedo_texture, uv0);
    buffer_material = texture(metallic_roughness_texture, uv0);
    buffer_geometry = vec4(uv0, 1.0, 0.5);
    
    // gl_FragDepth =  screen_pos.z / screen_pos.w;
}