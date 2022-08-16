#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform sampler2D image_texture_1;
uniform sampler2D image_texture_2;

void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    if (uv.x < 0.49) {
        FragColor = texture(image_texture_1, uv);
    } else if (uv.x > 0.51) {
        FragColor = texture(image_texture_2, uv);
    } else {
        FragColor = vec4(uv.xy, 0.0, 1.0);
    }
}

