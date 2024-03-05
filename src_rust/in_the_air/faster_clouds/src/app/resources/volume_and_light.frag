

in vec4 screen_pos;
out vec4 FragColor;

// The color buffer contains RGB albedo values
uniform sampler2D buffer_color;

// The geometry buffer contains the normal and distance from camera
uniform sampler2D buffer_geometry;

// The material buffer contains the metallic (r channel) and the roughness (g channel)
uniform sampler2D buffer_material;

uniform sampler2D cloud_map;

uniform sampler3D buffer_volume_noise;


uniform mat4 camera_to_screen;
uniform mat4 camera_to_world;
uniform mat4 world_to_camera;

uniform float time_since_start;




vec3 renderSky(vec3 direction) {
    float elevation = 1.0 - dot(direction, vec3(0,0,1));
    float centered = 1.0 - abs(1.0 - elevation);
    float sun_direction = dot(direction, LIGHT_DIRECTION);

    vec3 atmosphere_color = mix(AMBIENT_LIGHT, SUN_LIGHT, sun_direction * 0.5);
    
    vec3 base = mix(pow(AMBIENT_LIGHT, vec3(4.0)), atmosphere_color, pow(clamp(elevation, 0.0, 1.0), 0.5));
    float haze = pow(centered + 0.02, 4.0) * (sun_direction * 0.2 + 0.8);
    
    vec3 sky = mix(base, SUN_LIGHT, clamp(haze, 0.0, 1.0));
    
    float sun = pow(max((sun_direction - 29.0/30.0) * 30.0 - 0.05, 0.0), 6.0);
    
    return sky + sun;
}


vec4 light_surface(vec4 color, vec4 geometry, vec4 material, vec3 lightFromSunAtParticle) {
    vec3 normal = normalize(geometry.xyz);

    float diffuse = max(dot(normal, LIGHT_DIRECTION), 0.0);

    vec3 view_direction = normalize(camera_to_world[3].xyz - geometry.xyz);
    vec3 half_vector = normalize(LIGHT_DIRECTION + view_direction);
    float specular = pow(max(dot(normal, half_vector), 0.0), (material.g + 1.0) * 10.0);

    vec3 d = diffuse * color.rgb * lightFromSunAtParticle;
    vec3 s = specular * material.r * color.rgb * lightFromSunAtParticle;
    vec3 a = color.rgb * AMBIENT_LIGHT * AMBIENT_INTENSITY;

    return vec4(d + s + a, 1.0);
}



void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;


    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;
    mat4 world_to_screen = inverse(screen_to_world);

    vec4 ray_direction_screen = vec4(screen_pos.xy, 1.0, 1.0);
    vec4 ray_direction_camera = screen_to_camera * ray_direction_screen;
    vec4 ray_direction_world = camera_to_world * vec4(ray_direction_camera.xyz, 0.0);

    vec3 ray_start = camera_to_world[3].xyz;
    vec3 ray_direction = normalize(ray_direction_world.xyz);

    float dist_from_camera = 0.0;
    vec4 accumulation = vec4(0.0, 0.0, 0.0, 0.0);
    
    int steps_outside_cloud = 0;
    
    float noise = hash14(vec4(ray_direction * 1000.0, time_since_start * 10.0));
    

    // Backdrop
    vec4 backdrop = vec4(0.0);
    vec4 geometry = texture(buffer_geometry, uv);

    if (geometry.w == 0.0) {
        backdrop = vec4(renderSky(ray_direction), DRAW_DISTANCE);
    } else {
        float opaque_distance_from_camera = geometry.w;
        vec4 color = texture(buffer_color, uv);
        vec4 material = texture(buffer_material, uv);
        float materialTowardsSun = computeDensityTowardsSun(cloud_map, ray_start + ray_direction * opaque_distance_from_camera, 0.0);
        vec3 lightFromSunAtParticle = transmission(
            SUN_LIGHT * SUN_INTENSITY,
            materialTowardsSun
        );
        backdrop = vec4(
            light_surface(color, geometry, material, lightFromSunAtParticle).rgb,
            opaque_distance_from_camera
        );
    }
    
    // accumulation.rgb += beer(materialTowardsCamera * (1.0 - BASE_TRANSMISSION)) * backdrop.rgb;

    FragColor = backdrop;
}
