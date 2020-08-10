#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform sampler2D image_texture;

void main() {
    vec2 uv = screen_pos.xy * 0.5 - 0.5;
    FragColor = texture(image_texture, uv);
}

