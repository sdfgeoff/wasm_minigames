#version 300 es

precision mediump float;
in vec4 aVertexPosition;

out vec4 screen_pos;

void main() {
	screen_pos = aVertexPosition;
	gl_Position = screen_pos;
}
