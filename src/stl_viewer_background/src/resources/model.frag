#version 300 es

precision mediump float;
in vec3 screen_nor;
in vec3 world_nor;

out vec4 FragColor;

in mat4 camera_to_world;

uniform sampler2D image_matcap;
uniform vec3 color;


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
    vec3 diffuse = sample_background(world_nor, 3.0);
    
    vec3 reflect = reflect(vec3(0.0, 0.0, -1.0), screen_nor);
    vec4 reflect_world = (camera_to_world * vec4(reflect, 0.0));
    vec3 reflection = sample_background(reflect_world.xyz, 4.0);
    
    float fresnel = 1.0 - dot(screen_nor, vec3(0.0, 0.0, 1.0));
    
    vec3 out_col = color;
    out_col = out_col * diffuse;
    out_col += reflection * fresnel * 0.5;
    out_col *= 1.0 - fresnel * 0.5;
    
    FragColor.rgb = out_col;
    FragColor.a = 1.0;
}

