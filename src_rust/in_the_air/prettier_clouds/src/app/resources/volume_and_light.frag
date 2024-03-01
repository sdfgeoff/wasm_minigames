#version 300 es

precision mediump float;
precision mediump sampler3D;
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


const float WORLD_SCALE = 0.05; // Scale all the cloud parameters by this amount

const float CLOUD_MAP_EXTENT = 128.0 * 500.0 * WORLD_SCALE; // 128 pixels, 200 meters per pixel
const vec4 CLOUD_LAYER_HEIGHTS = vec4(0.0, 1.0, 2.0, 4.0) * 1500.0 * WORLD_SCALE;
const float CLOUD_LAYER_THICKNESS = 1650.0 * WORLD_SCALE; // If this is bigger than the distance between the gap between CLOUD_LAYER_HEIGHTS then the clouds can overlap
const float CLOUD_UNDERHANG = CLOUD_LAYER_THICKNESS * 0.5; // How much the cloud layer extends below the layer height
const float CLOUD_NOISE_SCALE = WORLD_SCALE * 0.1;
const vec3 CLOUD_NOISE_SPEED = vec3(0.02, 0.0, 0.0);
const int CLOUD_NOISE_OCTAVES = 1;
const float CLOUD_NOISE_DENSITY_VARIATION = 30.0;


// Raymarcher Parameters
const int MAX_STEPS = 128;
const float DRAW_DISTANCE = 2000.0;
const float INSIDE_STEP_SIZE = 3.0;
const float OUTSIDE_STEP_SIZE = INSIDE_STEP_SIZE * 4.0;
const int STEP_OUTSIDE_RATIO = int(ceil(INSIDE_STEP_SIZE / OUTSIDE_STEP_SIZE));

// Cloud Material Parameters
const float CLOUD_DENSITY_SCALE = 0.04;

const float kb = 1.0; // Backscattering
const float kbp = 30.0; // Backscattering falloff

const float ks = 0.8; // Onmidirectional Scattering
const float kt = 1.0; // Transmission Scattering
const float ktp = 2.0; // Transmission falloff

const float BASE_TRANSMISSION = 0.97; // Light that doesn't get scattered at all


// Lighting Parameters
const vec3 LIGHT_DIRECTION = normalize(vec3(0,1.0,0.5));
const vec3 SUN_LIGHT = vec3(0.99, 0.97, 0.96);
const vec3 AMBIENT_LIGHT = vec3(0.52,0.80,0.92);
const float AMBIENT_INTENSITY = 0.2; // How "strong" is the ambient light
const float SUN_INTENSITY = 1.2; // How "strong" is the sun


const float E = 2.718;


float hash14(vec4 p4)
{
	p4 = fract(p4  * vec4(.1031, .1030, .0973, .1099));
    p4 += dot(p4, p4.wzxy+33.33);
    return fract((p4.x + p4.y) * (p4.z + p4.w));
}


float beerPowder(float material_amount) {
    return pow(E, -material_amount) - pow(E, -material_amount * material_amount);
}

float beer(float material_amount) {
    return pow(E, -material_amount);
}


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



float sampleCloudMapShape(vec3 point) {
    point.x += time_since_start;
    vec4 map_sample = (textureLod(cloud_map, point.rg / CLOUD_MAP_EXTENT, 0.0) - 0.5) * 2.0;

    vec4 layer_density = map_sample;
    vec4 layer_centerline = CLOUD_LAYER_HEIGHTS + (CLOUD_LAYER_THICKNESS - CLOUD_UNDERHANG) * layer_density;
    vec4 layer_thickness = max(CLOUD_LAYER_THICKNESS * layer_density, 0.0);
    vec4 distance_to_centerline = abs(point.z - layer_centerline);
    vec4 distance_to_surface = distance_to_centerline - layer_thickness;
    vec4 distance_to_layer = distance_to_surface;

    float distance_to_cloud = min(min(min(distance_to_layer.x, distance_to_layer.y), distance_to_layer.z), distance_to_layer.w);

    float density = -distance_to_cloud;
    return density * CLOUD_DENSITY_SCALE;
}




float computeDensityTowardsSun(vec3 current_position, float density_here) {
    float density_sunwards = max(density_here, 0.0);
    density_sunwards += max(0.0, sampleCloudMapShape(current_position + LIGHT_DIRECTION * 1.0)) * 60.0 * WORLD_SCALE;
    density_sunwards += max(0.0, sampleCloudMapShape(current_position + LIGHT_DIRECTION * 4.0)) * 240.0 * WORLD_SCALE;
    
    return density_sunwards;
}


vec3 transmission(vec3 light,float material_amount) {
    return beer(material_amount * (1.0 - BASE_TRANSMISSION)) * light;
}

vec3 lightScattering(vec3 light, float angle, float material_amount) {
    // Compute the color/intensity of the light scattering in a particular direction
    // Angle ranges from 1.0 (transmission/forward scattering) to -1.0 (back scattering)  
    

    
    angle = (angle + 1.0) * 0.5; // Angle between 0 and 1
  
  
    float ratio = 0.0;
    ratio += kb * pow(1.0 - angle, kbp);
    ratio += kt * pow(angle, ktp);
    ratio = ratio * (1.0 - ks) + ks;
    
    
    /*float ratio = 0.0;
    ratio = (1.0 - smoothstep(0.0, 0.5,(1.0 - angle) * ktp)) * kt;
    ratio += (1.0 - smoothstep(0.0, 0.5, (angle) * kbp)) * kb;
    
    ratio = ratio * (1.0 - ks) + ks;*/
    light = light * ratio * (1.0 - BASE_TRANSMISSION);
    
    // Transmit....
    return light;
}


float addNoiseToDensity(vec3 point, float density, int octaves) {
    for (int j = 0; j < octaves; j++) {
        float level = float(j) + 1.0;
        float l2 = level * level;
        float scale = CLOUD_NOISE_SCALE * level;
        vec3 position_offset = time_since_start * CLOUD_NOISE_SPEED * l2;
        vec4 small_noise_tex = textureLod(buffer_volume_noise, point * scale + position_offset, 0.0);
        density -= pow(small_noise_tex.r, 2.0) * CLOUD_NOISE_DENSITY_VARIATION  * CLOUD_DENSITY_SCALE * density;
    }
    return density;
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

vec4 alphaOver(vec4 top, vec4 bottom) {
    float A1 = bottom.a * (1.0 - top.a);

    float A0 = top.a + A1;
    return vec4((top.rgb * top.a + bottom.rgb * A1) / A0, A0);
}


void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    vec4 geometry = texture(buffer_geometry, uv);
    float opaque_distance_from_camera = geometry.w == 0.0 ? 10000.0 : geometry.w;

    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;
    mat4 world_to_screen = inverse(screen_to_world);

    vec4 ray_direction_screen = vec4(screen_pos.xy, 1.0, 1.0);
    vec4 ray_direction_camera = screen_to_camera * ray_direction_screen;
    vec4 ray_direction_world = camera_to_world * vec4(ray_direction_camera.xyz, 0.0);

    vec3 ray_start = camera_to_world[3].xyz;
    vec3 ray_direction = normalize(ray_direction_world.xyz);

    float dist_from_camera = 0.0;
    vec4 outCol = vec4(0.0, 0.0, 0.0, 0.001); // This small starting opacity prevents a div-zero error in alpha compositing


    vec4 accumulation = vec4(0.0, 0.0, 0.0, 0.0);
    
    int steps_outside_cloud = 0;
    
    float noise = hash14(vec4(ray_direction * 1000.0, time_since_start * 10.0));
    
    vec3 sky = renderSky(ray_direction);
    
    
    float materialTowardsCamera = 0.0;


    for (int i=0; i<MAX_STEPS; i+=1) {
        vec3 current_position = ray_start + (dist_from_camera + noise * INSIDE_STEP_SIZE) * ray_direction;
        
        float cloud_map = sampleCloudMapShape(current_position);
        
        if (cloud_map > 0.0) {
            if (steps_outside_cloud != 0) {
                // First step into the cloud;
                steps_outside_cloud = 0;
                dist_from_camera = dist_from_camera - OUTSIDE_STEP_SIZE + INSIDE_STEP_SIZE;
                continue;
            }
            steps_outside_cloud = 0;
            
        } else {
            steps_outside_cloud += 1;
        }
        
        float step_size = OUTSIDE_STEP_SIZE;
        
        if (steps_outside_cloud <= STEP_OUTSIDE_RATIO && cloud_map > 0.0) {  
            step_size = INSIDE_STEP_SIZE;
            
            float density_here = cloud_map;

            // We only need to sample the detailed cloud texture if
            // we are close and can see it in lots of detail.
            if (dist_from_camera < DRAW_DISTANCE / 3.0) {
                // If we are already mostly opaque, there's no point sampling extra-detail.
                //vec4 small_noise_tex = textureLod(BUFFER_VOLUME_NOISE, current_position * 0.05 + vec3(0,iTime * 0.02,0), 0.0);
                //density -= pow(small_noise_tex.r, 3.0) * 3.0;
                density_here = addNoiseToDensity(current_position, density_here, CLOUD_NOISE_OCTAVES);
            }
            
            density_here = smoothstep(0.0, 0.1, density_here);
            
            density_here = max(density_here, 0.0);
            float material_here = density_here * step_size;
            materialTowardsCamera += material_here;
            
            float materialTowardsSun = computeDensityTowardsSun(current_position, density_here);
            
            vec3 lightFromSunAtParticle = transmission(
                SUN_LIGHT * SUN_INTENSITY,
                materialTowardsSun
            );
                        
            float angleToSun = dot(ray_direction, LIGHT_DIRECTION);

            vec3 lightAtParticle = lightFromSunAtParticle;
            vec3 lightScatteringTowardsCamera = lightScattering(
                lightAtParticle * material_here,
                angleToSun,
                materialTowardsCamera
            );
            vec3 lightReachingCamera = transmission(
                lightScatteringTowardsCamera,
                materialTowardsCamera
            );
            accumulation.rgb += lightReachingCamera;
        }

        if (materialTowardsCamera * CLOUD_DENSITY_SCALE > 4.0) {
            break;
        }
        

        dist_from_camera += step_size;
        if (dist_from_camera > opaque_distance_from_camera) {
            vec4 color = texture(buffer_color, uv);
            vec4 material = texture(buffer_material, uv);
            // Replace the sky with the surface
            float materialTowardsSun = computeDensityTowardsSun(current_position, 1.0);
            vec3 lightFromSunAtParticle = transmission(
                SUN_LIGHT * SUN_INTENSITY,
                materialTowardsSun
            );
            sky = light_surface(color, geometry, material, lightFromSunAtParticle).rgb;
            //outCol = alphaOver(outCol, light_surface(color, geometry, material));

            break;
        }
        else if (dist_from_camera > DRAW_DISTANCE) {
            break;
        }
    }
    

    accumulation.rgb += beer(materialTowardsCamera * (1.0 - BASE_TRANSMISSION)) * sky;
    


    // for(int i = 0; i < 100; i++) {
    //     float step_size = 5.0;
    //     vec3 p1 = ray_start + ray_direction * dist_from_camera;

    //     float density = sample_volume_density(p1);
    //     float absorbtion = min(max(density * step_size, 0.0), 1.0);

    //     if(dist_from_camera > opaque_distance_from_camera) {
    //         // We've hit something opaque, so light the surface and call
    //         // it done.

    //         vec4 color = texture(buffer_color, uv);
    //         vec4 material = texture(buffer_material, uv);
    //         outCol = alphaOver(outCol, light_surface(color, geometry, material));

    //         break;
    //     }

    //     vec4 color = vec4(0.2, 0.2, 0.3, absorbtion);
    //     outCol = alphaOver(outCol, color);

    //     if(outCol.a > 0.99) {
    //         // Fully opaque so anything behind this isn't visible anyway
    //         break;
    //     }

    //     if(dist_from_camera > 5000.0) {
    //         // We've rendered far enough away
    //         break;
    //     }

    //     dist_from_camera += step_size;// * max(distance_to_cloud, 1.0);
    // }

    // Put in a black background:
    // outCol = alphaOver(outCol, vec4(0.0, 0.0, 0.0, 1.0));

    FragColor = accumulation;
}
