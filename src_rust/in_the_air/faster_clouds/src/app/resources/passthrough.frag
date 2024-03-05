#version 300 es

precision highp float;
precision highp int;

in vec4 screen_pos;
out vec4 FragColor;

uniform vec2 resolution;

uniform sampler2D lighting_texture;
uniform sampler2D volume_texture;

const float BASE_TRANSMISSION = 0.97; // Light that doesn't get scattered at all

const float E = 2.718;



float beer(float material_amount) {
    return pow(E, -material_amount);
}


vec4 sample_volume(in vec2 uv, out float depth) {
    vec4 raw = texture(volume_texture, uv);
    uint data_uint = floatBitsToUint(raw.a);
    vec2 data = unpackHalf2x16(data_uint);

    depth = data.y;
    float mat = data.x;

    return vec4(
        raw.rgb,
        mat
    );
}


void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    vec4 light = texture(lighting_texture, uv);

    float surfaceDepth = light.a;

    vec2 offset = 1.0 / resolution * 2.0;
    float depth1, depth2, depth3, depth4 = 0.0;
    vec4 v1 = sample_volume(uv + offset * vec2(-1, 0), depth1);
    vec4 v2 = sample_volume(uv + offset * vec2(1, 0), depth2);
    vec4 v3 = sample_volume(uv + offset * vec2(0, 1), depth3);
    vec4 v4 = sample_volume(uv + offset * vec2(0, -1), depth4);

    // Find out which volume sample that is nearest to the surfae depth
    vec4 deltas = abs(vec4(surfaceDepth) - vec4(depth1, depth2, depth3, depth4));
    float minDelta = min(min(min(deltas.x, deltas.y), deltas.z), deltas.w);

    vec4 volume = v1;
    if (minDelta == deltas.x) {
        volume = v1;
    } else if (minDelta == deltas.y) {
        volume = v2;
    } else if (minDelta == deltas.z) {
        volume = v3;
    } else if (minDelta == deltas.w) {
        volume = v4;
    }
    
    float materialTowardsCamera = volume.a;

    vec3 color = volume.rgb + beer(materialTowardsCamera * (1.0 - BASE_TRANSMISSION)) * light.rgb;

    FragColor = vec4(color.rgb, 1.0);
}

