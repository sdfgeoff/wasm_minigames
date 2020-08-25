#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

const float track_base_radius = 0.5;
const float track_width = 0.1;

const float track_background_grid_spacing = 5.0;
const float track_background_line_fade = 0.04;
const float track_background_line_width = 1.0;
const float track_edge_line_width = 0.5;


vec4 sin_consts_1 = vec4(0.2, 0.0, 0.0, 0.0);
vec4 sin_consts_2 = vec4(0.0, 0.0, 0.0, 0.0);
vec4 cos_consts_1 = vec4(0.0, -0.2, 0.0, 0.1);
vec4 cos_consts_2 = vec4(0.0, 0.0, 0.05, 0.0);


float map_function(vec2 position) {
    float course = length(position - vec2(0.0, 0.0));
    
    float angle = atan(position.x, position.y);
    vec4 angles_1 = vec4(angle, angle*2.0, angle*3.0, angle*4.0);
    vec4 angles_2 = vec4(angle*5.0, angle*6.0, angle*7.0, angle*8.0);
    
    float track_radius = track_base_radius;

    track_radius += dot(sin(angles_1), sin_consts_1);
    track_radius += dot(sin(angles_2), sin_consts_2);
    track_radius += dot(cos(angles_1), cos_consts_1);
    track_radius += dot(cos(angles_2), cos_consts_2);

    float track_sdf = course - track_radius;
    track_sdf = abs(track_sdf) - track_width;
    return track_sdf;
}



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


float background_grid(vec2 world_coordinates) {
    vec2 sections = mod(world_coordinates * track_background_grid_spacing, 1.0);
    sections = abs(0.5 - sections);
    vec2 lines = sections + track_background_line_fade;
    lines /= track_background_line_width;
    return min(lines.x, lines.y);
}

float map_edges(float track) {
    return abs(track) / track_edge_line_width;
}


void main() {
    float track = map_function(uv);
    
    float edge_sdf = map_edges(track);
    float background_grid = background_grid(uv);
    
    float map_visualized = edge_sdf;
    if (track > 0.0) {
        map_visualized = min(edge_sdf, background_grid);
    }
    
    
    FragColor = neon(
        map_visualized,
        vec4(0.9, 0.9, 0.9, 1.0), 0.1
    );
}

