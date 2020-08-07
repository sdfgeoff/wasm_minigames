#version 300 es

precision mediump float;
in vec4 aVertexPosition;

out vec4 screen_pos;

void main() {
	mat4 trans = mat4(
		vec4(1.0, 0.0, 0.0, 0.0),
		vec4(0.0, 1.0, 0.0, 0.0),
		vec4(0.0, 0.0, 1.0, 0.0),
		vec4(0.0, 0.0, 0.0, 1.0)
	);
	
	screen_pos = aVertexPosition * trans;
	gl_Position = screen_pos;
}
