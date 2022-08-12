#version 300 es

precision mediump float;
in vec2 uv;
out vec4 FragColor;

float track_radius = 0.5;

void main() {
        vec2 position = uv;
        float course = length(position - vec2(0.0, 0.0));
        float track_sdf = course - track_radius;
        
        FragColor = vec4(vec3(track_sdf > 0.0), 1.0);
}

