#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

const float track_base_radius = 0.5;
const float track_width = 0.1;

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

void main() {
    float track = map_function(uv);
    FragColor = vec4(vec3(track > 0.0), 1.0);
}

