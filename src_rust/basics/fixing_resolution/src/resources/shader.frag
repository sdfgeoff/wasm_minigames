#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;


void main() {
	FragColor = vec4(
		screen_pos.x,
		screen_pos.y,
		1.0,
		1.0
	);
}
