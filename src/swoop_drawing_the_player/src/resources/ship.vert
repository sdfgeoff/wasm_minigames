#version 300 es

precision mediump float;
in vec4 aVertexPosition;

out vec2 uv;

void main() {
	vec4 screen_pos = aVertexPosition;
	uv = screen_pos.xy;
	gl_Position = screen_pos;
}
