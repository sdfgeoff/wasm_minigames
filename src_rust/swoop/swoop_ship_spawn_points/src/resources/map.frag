#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

const float track_background_grid_spacing = 5.0;
const float track_background_line_fade = 0.04;
const float track_background_line_width = 1.0;
const float track_edge_line_width = 0.5;


uniform float track_base_radius;
uniform float track_width;
uniform float sin_consts[8];
uniform float cos_consts[8];

uniform vec2 start_line_tangent;
uniform vec2 start_line_position;


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

float startline(vec2 world_coordinates) {
    vec2 delta = world_coordinates - start_line_position;
    float projected_dist = dot(delta, start_line_tangent);
    
    vec2 start_line_coords = delta - projected_dist * start_line_tangent;
    float dist_from_line = length(start_line_coords);
    float dist_from_center = projected_dist;
    
    float start_line_ends = - 1.0 + abs(dist_from_center);
    
    float start_line = max(dist_from_line, start_line_ends);
    
    return start_line + track_background_line_fade;
}


void main() {
    float track = map_function(uv);
    
    float edge_sdf = map_edges(track);
    
    
    float map_visualized = edge_sdf;
    if (track > 0.0) {
        float background_grid = background_grid(uv);
        map_visualized = min(edge_sdf, background_grid);
    } else {
        float startline_sdf = startline(uv);
        map_visualized = min(edge_sdf, startline_sdf);
    }
    
    
    FragColor = neon(
        map_visualized,
        vec4(0.9, 0.9, 0.9, 1.0), 0.1
    );
}

