#version 300 es

precision mediump float;
in vec4 attribute_vertex_position;
in vec4 attribute_vertex_normal;
in vec2 attribute_vertex_uv0;

out vec4 screen_pos;
out vec2 uv0;

void main() {
    uv0 = attribute_vertex_uv0.xy;
	screen_pos = attribute_vertex_position;
	gl_Position = screen_pos;
}
