#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

void main() {
	FragColor = vec4(uv, 1.0, 1.0);
}

