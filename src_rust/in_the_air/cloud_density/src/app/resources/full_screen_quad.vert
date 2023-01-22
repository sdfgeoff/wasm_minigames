#version 300 es

precision mediump float;
in vec4 attribute_vertex_position;

out vec4 screen_pos;

void main() {
	screen_pos = attribute_vertex_position;
	gl_Position = screen_pos;
}
