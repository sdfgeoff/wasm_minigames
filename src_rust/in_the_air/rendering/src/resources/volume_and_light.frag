#version 300 es

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform sampler2D buffer_color;
uniform sampler2D buffer_geometry;
uniform sampler2D buffer_material;

void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    vec4 color = texture(buffer_color, uv);
    vec4 geometry = texture(buffer_geometry, uv);
    vec4 material = texture(buffer_material, uv);

    vec4 outCol = vec4(0.0, 0.0, 0.0, 1.0);

    outCol += color * sin(0.5 * 3.14159 * 2.0 * 0.0);
    outCol += geometry * sin(0.5 * 3.14159 * 2.0 * 0.33333);
    outCol += material * sin(0.5 * 3.14159 * 2.0 * 0.66666);

    FragColor = outCol;
}

