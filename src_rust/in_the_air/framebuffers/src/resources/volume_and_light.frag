#version 300 es

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform sampler2D buffer_color;
uniform sampler2D buffer_geometry;
uniform sampler2D buffer_material;

void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    if (uv.x < 0.33333) {
        FragColor = texture(buffer_color, uv);
    } else if (uv.x < 0.6666) {
        FragColor = texture(buffer_geometry, uv);
    } else {
        FragColor = texture(buffer_material, uv);
    }
}

