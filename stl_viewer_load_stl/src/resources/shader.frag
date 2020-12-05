#version 300 es

precision mediump float;
in vec4 screen_pos;
in vec4 screen_nor;
out vec4 FragColor;

uniform sampler2D image_matcap;

void main() {
    vec2 matcap_coords = screen_nor.xy * 0.5 + vec2(0.5);
    vec4 matcap = texture(image_matcap, matcap_coords);

    FragColor = matcap;
}

