#version 300 es

precision mediump float;
in vec2 vert_pos;
in vec2 vert_nor;
in vec2 vert_uv0;


out vec2 uv0;


void main() {
    
    uv0 = vert_uv0;
    gl_Position.xy = vert_pos.xy;
    gl_Position.z = 0.99999;
    gl_Position.w = 1.0;
}
