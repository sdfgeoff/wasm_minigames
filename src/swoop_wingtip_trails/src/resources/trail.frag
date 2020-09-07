#version 300 es

precision mediump float;
in vec2 uv;
in vec4 data;
in float trail_percent;
in float segment_percent;
out vec4 FragColor;


uniform vec4 trail_color;
uniform float trail_brightness;


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
    float intensity = data.x;
    
    float sdf_normal = 1.0 - abs(uv.x);
    float sdf_tangent = 1.0 - trail_percent;
    
    float falloff = 1.0 - sdf_normal * sdf_tangent;
    falloff += (1.0 - intensity);
    
    FragColor = neon(falloff, trail_color, 1.0) * data.y;
}

