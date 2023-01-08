#version 300 es

precision mediump float;

layout(location=0) out vec4 buffer_color;
layout(location=1) out vec4 buffer_geometry;
layout(location=2) out vec4 buffer_material;

void main() {
    buffer_color = vec4(0,1,0.2,0.5);
    buffer_material = vec4(1,0.2,0.1,0.5);
    buffer_geometry = vec4(0.5, 0.5, 1.0, 0.5);
    
    // gl_FragDepth =  screen_pos.z / screen_pos.w;
}