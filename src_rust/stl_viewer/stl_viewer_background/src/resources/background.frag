#version 300 es

precision mediump float;

in vec3 world_pos;

out vec4 FragColor;

uniform sampler2D image_matcap;


vec3 sample_background(vec3 pos, float mip) {
    vec2 coords = vec2(
        atan(pos.y, pos.x) / 3.14159,
        sin(pos.z)
    );
    coords = coords * 0.5 + 0.5;
    
    vec4 raw = textureLod(image_matcap, coords, mip);
    float luminance = pow(((1.0 / raw.a) - 1.0), 1.0 / 2.2);
    
    return raw.rgb * luminance;
}


void main() {
    vec3 matcap = sample_background(world_pos, 2.0);

    FragColor.rgb = matcap * 0.5;
    FragColor.a = 1.0;
}

