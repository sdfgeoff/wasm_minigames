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


const float sin_consts[8] = float[8](0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
const float cos_consts[8] = float[8](0.0, -0.2, 0.0, 0.1, 0.0, 0.0, 0.05, 0.0);


float map_function(vec2 position) {
    float course = length(position - vec2(0.0, 0.0));
    float angle = atan(position.x, position.y);
    float track_radius = track_base_radius;
    
    for (int i=0; i<8; i++) {
        float omega = float(i+1);
        track_radius += cos(angle * omega) * cos_consts[i];
        track_radius += sin(angle * omega) * sin_consts[i];
    }

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

