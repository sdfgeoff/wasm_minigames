#version 300 es

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform sampler2D input_texture;

void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;
    FragColor = texture(input_texture, uv);
}

