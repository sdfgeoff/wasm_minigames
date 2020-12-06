#version 300 es

precision mediump float;
precision highp int;

in vec2 uv;
out vec4 FragColor;

// Texture to sample to get glyphs
uniform sampler2D font_texture;
// How many tiles there are in the font texture
const ivec2 TILES = ivec2(10, 7);
// Size of a character in the tile sheet
const vec2 CHARACTER_SIZE = vec2(1.0/10.0, 1.0/7.0);

uniform vec4 text_data[64];
uniform ivec2 text_box_dimensions;



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

vec4 get_sprite(vec2 uv, vec2 offset, vec2 size) {
        return texture(font_texture, uv * size + offset);
}

float get_character(vec2 uv, int character) {
        vec2 offset = vec2(ivec2(
                character % TILES.x,
                character / TILES.x
        )) * CHARACTER_SIZE;
        
        vec2 size = CHARACTER_SIZE;
        vec4 channel = vec4(0.0, 1.0, 0.0, 0.0);
        
        if (character == -1) {
                size = vec2(1.0);
                offset = vec2(0.0);
                channel = vec4(1.0, 0.0, 0.0, 0.0);
        }
        vec4 color = get_sprite(uv, offset, size);
        return dot(color, channel);
}

void main() {
        vec2 coord = uv * 0.5 + 0.5;

        coord.x *= float(text_box_dimensions.x);
        coord.y *= float(text_box_dimensions.y);
        int letter_id = int(coord.x) + (text_box_dimensions.y - int(coord.y) - 1) * text_box_dimensions.x;
        coord.x -= floor(coord.x);
        coord.y -= floor(coord.y);
        
        vec4 char_data = text_data[letter_id];
        
        float char_sdf = get_character(coord, int(char_data.a));
        FragColor = neon(
                1.0 - smoothstep(0.0, 0.55, char_sdf),
                vec4(char_data.rgb, 1.0),
                1.0
        );
}

