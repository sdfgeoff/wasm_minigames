#version 300 es

precision mediump float;

in vec2 uv0;
out vec4 FragColor;
in mat4 camera_to_world;

uniform sampler2D image_albedo;
uniform sampler2D image_normal_depth;
uniform sampler2D image_depth;

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
    
    vec4 albedo = texture(image_albedo, uv0);
    vec4 normal_depth = texture(image_normal_depth, uv0);
    
    float depth = normal_depth.a;
    float fog = pow(depth, 0.5) * 1.5;
    
    vec3 lighting = sample_background(normal_depth.xyz, 3.0);
    
    vec3 out_col = lighting * albedo.rgb;
    
    out_col = mix(out_col, vec3(1.0, 1.0, 1.0), fog);
    
    FragColor.rgb = out_col;
    FragColor.a = 1.0;
    
    FragColor.rgb = vec3(texture(image_depth, uv0).r);
}

