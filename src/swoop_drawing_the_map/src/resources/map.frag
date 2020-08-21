#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

uniform sampler2D ship_texture;
uniform float ship_engine;
uniform vec4 ship_color;

vec4 neon(float sdf, vec4 color, float glow_width) {
	float ramp = clamp(1.0 - sdf / glow_width, 0.0, 1.0);
	vec4 outp = vec4(0.0);
	ramp = ramp * ramp;
	outp += pow(color, vec4(4.0)) * ramp;
	ramp = ramp * ramp;
	outp += color * ramp;
	ramp = ramp * ramp;
	outp += vec4(1.0) * ramp;
	return outp;
}

void main() {
	FragColor = vec4(uv, 1.0, 1.0);
}

