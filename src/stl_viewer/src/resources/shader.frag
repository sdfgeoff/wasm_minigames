#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec4 screen_pos;
in vec4 screen_nor;
out vec4 FragColor;

uniform sampler2D image_background;
uniform sampler2D image_matcap;

void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;
    
    vec2 matcap_coords = screen_nor.xy * 0.5 + vec2(0.5);
    vec4 matcap = texture(image_matcap, matcap_coords);

    FragColor = matcap;
    FragColor.a = 1.0;
}

