#version 300 es
// Draw the ship

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
	
	vec4 raw_sprite = texture(ship_texture, uv);
	
	FragColor = neon(0.9 - raw_sprite.r, ship_color, 1.0);
	vec4 engine_color = ship_engine * ship_color;
	FragColor += neon(0.9 - raw_sprite.b, engine_color, 1.0);
}

