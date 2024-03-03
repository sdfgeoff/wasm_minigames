#version 300 es

precision mediump float;
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

vec4 alphaOver(vec4 top, vec4 bottom) {
    float A1 = bottom.a * (1.0 - top.a);

    float A0 = top.a + A1;
    return vec4((top.rgb * top.a + bottom.rgb * A1) / A0, A0);
}


void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    vec4 light = texture(lighting_texture, uv);
    vec2 offset = 1.0 / resolution * 4.0;
    vec4 v1 = texture(volume_texture, uv + offset);
    vec4 v2 = texture(volume_texture, uv + offset * vec2(1, 0));
    vec4 v3 = texture(volume_texture, uv + offset * vec2(0, 1));
    vec4 v4 = texture(volume_texture, uv);

    vec4 volume = (v1 + v2 + v3 + v4) * 0.25;


    float materialTowardsCamera = volume.a;

    vec3 color = volume.rgb + beer(materialTowardsCamera * (1.0 - BASE_TRANSMISSION)) * light.rgb;

    //vec4 color = alphaOver(volume, vec4(light.rgb, 1.0));

    FragColor = vec4(color.rgb, 1.0);
}

