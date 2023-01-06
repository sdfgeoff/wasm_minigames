#version 300 es

precision highp float;

uniform mat4 world_to_camera;
uniform mat4 camera_to_screen;

in vec2 uv0;

layout(location=0) out vec4 normal_depth;
layout(location=1) out vec4 albedo;

uniform sampler2D image_world_texture;


const float camera_near = 0.1;
const float camera_far = 500.0;

const float world_texture_size = 1024.0;
const float world_scale = 4.0 / world_texture_size;

const mat3 tex_scaling_factors = mat3(
    vec3(1.0, 3.0, 0.0) * 1.0, // Left
    vec3(1.0, 3.0, 0.0) * 1.295,  // Front
    vec3(1.0, 1.0, 0.0) * 0.7345 // Top
) * world_scale;

// Raymarcher Configuration
const int max_steps = 120;
const float max_draw_distance = camera_far;
const float step_scale = 8.0; // A full white pixel results in a step of this many world units
const float NORMAL_SAMPLE_SIZE = world_scale * world_texture_size * 0.05; // Size of tetrahedron to use for sampling normal
const float tolerance = 0.001;


const vec3 FOG_COLOR = vec3(1.0, 0.9, 0.8);
const vec3 SURFACE_COLOR = vec3(0.7, 0.5, 0.2);
const vec3 UNDERSIDE_COLOR = vec3(0.6, 0.2, 0.1);



float sample_tex(vec3 coord) {
    // Pretend our texture contains a SDF in each axis.
    // Munge some of the sample coordinates to reduce periodicity
    
    
    float left = textureLod(image_world_texture, coord.yz * tex_scaling_factors[0].xy, 0.0).g;
    float front = textureLod(image_world_texture, coord.xz * tex_scaling_factors[1].xy, 0.0).b;
    float top = textureLod(image_world_texture, coord.xy * tex_scaling_factors[2].xy, 0.0).r;
    
    return (top + left + front) / 3.0;
}


float map(vec3 coord) {
    coord += vec3(25.0, 110.0, 6.0);

    float rocks = 0.5 - sample_tex(coord);
    return rocks;
}




/// Raymarch by the distance field each step until the step count or
/// the maximum distance is reached.
vec4 raymarch(vec3 start_point, vec3 direction, int steps, float max_dist) {
    vec3 position = start_point;

    
    float dist = 0.0;
    
    for (int i=0; i<steps; i++) {
        float df = map(position);
        
        float threshold = tolerance;
        float step_size = df * step_scale; // This helps because it isn't a true SDF
        
        if (abs(df) < threshold) {
            return vec4(position, dist / max_dist);
        }
        if  (dist > max_dist) {
            return vec4(position, 1.0);
        }
        dist += step_size;
        position += direction * step_size;
    }
    return vec4(position, dist/max_dist);
}


vec3 calc_normal(vec3 sample_point) {
    const float h = NORMAL_SAMPLE_SIZE; // replace by an appropriate value
    const vec2 k = vec2(1,-1);
    
    vec3 normal = normalize(
		k.xyy * map( sample_point + k.xyy*h ) + 
		k.yyx * map( sample_point + k.yyx*h ) + 
		k.yxy * map( sample_point + k.yxy*h ) + 
		k.xxx * map( sample_point + k.xxx*h ) );
    //normal = normal.zyx;
    return normal;
}

vec3 texture_surface(vec3 position, vec3 normal) {
    vec4 noise = texture(image_world_texture, (position.xy * 1.0 + normal.x * 0.1) * 0.1);
    
    vec3 col = mix(UNDERSIDE_COLOR, SURFACE_COLOR, normal.z + normal.x * normal.y);
    col = mix(col, noise.rgb, 0.5);
    
    return col;
}



void main() {
    mat4 camera_to_world = inverse(world_to_camera);
    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;
    mat4 world_to_screen = inverse(screen_to_world);
    
    vec2 vert_pos = uv0 * 2.0 - 1.0;
    
    vec4 ray_direction_screen = vec4(vert_pos, 1.0, 1.0);
    vec4 ray_direction_camera = screen_to_camera * ray_direction_screen;
    vec4 ray_direction_world = camera_to_world * ray_direction_camera;
    
    vec3 ray_start = camera_to_world[3].xyz;
    vec3 ray_direction = normalize(ray_direction_world.xyz);
    
    ray_start += ray_direction * camera_near; // So that a march distance of zero is the camera near plane

    vec4 data = raymarch(ray_start, ray_direction, max_steps, max_draw_distance);
    
    vec3 surface_normal = calc_normal(data.xyz);
    
    vec3 surface_color = texture_surface(data.xyz, surface_normal);
    
    albedo.rgb = surface_color;
    
    normal_depth = vec4(surface_normal, data.a * max_draw_distance);

    vec3 intersection_point = data.xyz; 
    vec4 screen_pos = world_to_screen * vec4( intersection_point, 1.0 );
    gl_FragDepth = screen_pos.z / screen_pos.w;

}

