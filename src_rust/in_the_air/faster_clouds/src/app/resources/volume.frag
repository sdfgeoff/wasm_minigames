
precision mediump sampler3D;

in vec4 screen_pos;
out vec4 FragColor;

// The geometry buffer contains the normal and distance from camera
uniform sampler2D buffer_geometry;

uniform sampler2D cloud_map;

uniform sampler3D buffer_volume_noise;

uniform mat4 camera_to_screen;
uniform mat4 camera_to_world;
uniform mat4 world_to_camera;

uniform float time_since_start;



vec3 lightScattering(vec3 light, float angle, float material_amount) {
    // Compute the color/intensity of the light scattering in a particular direction
    // Angle ranges from 1.0 (transmission/forward scattering) to -1.0 (back scattering)  

    angle = (angle + 1.0f) * 0.5f; // Angle between 0 and 1

    float ratio = 0.0f;
    ratio += kb * pow(1.0f - angle, kbp);
    ratio += kt * pow(angle, ktp);
    ratio = ratio * (1.0f - ks) + ks;

    light = light * ratio * (1.0f - BASE_TRANSMISSION);

    return light;
}

float addNoiseToDensity(vec3 point, float density, int octaves) {
    for(int j = 0; j < octaves; j++) {
        float level = float(j) + 1.0f;
        float l2 = level * level;
        float scale = CLOUD_NOISE_SCALE * level;
        vec3 position_offset = time_since_start * CLOUD_NOISE_SPEED * l2;
        vec4 small_noise_tex = textureLod(buffer_volume_noise, point * scale + position_offset, 0.0f);
        density -= pow(small_noise_tex.r, 2.0f) * CLOUD_NOISE_DENSITY_VARIATION * CLOUD_DENSITY_SCALE * density;
    }
    return density;
}

void main() {
    vec2 uv = screen_pos.xy * 0.5f + 0.5f;

    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;

    vec4 ray_direction_screen = vec4(screen_pos.xy, 1.0f, 1.0f);
    vec4 ray_direction_camera = screen_to_camera * ray_direction_screen;
    vec4 ray_direction_world = camera_to_world * vec4(ray_direction_camera.xyz, 0.0f);

    vec3 ray_start = camera_to_world[3].xyz;
    vec3 ray_direction = normalize(ray_direction_world.xyz);

    float dist_from_camera = 0.0f;
    vec3 accumulation = vec3(0.0f, 0.0f, 0.0f);

    int steps_outside_cloud = 0;

    float noise = hash14(vec4(ray_direction * 1000.0f, time_since_start * 10.0f));

    // Backdrop
    vec4 geometry = texture(buffer_geometry, uv);
    float max_distance = geometry.w == 0.0f ? DRAW_DISTANCE : geometry.w;

    float materialTowardsCamera = 0.0f;

    float steps = 0.0f;

    for(int i = 0; i < MAX_STEPS; i += 1) {
        steps = float(i);
        vec3 current_position = ray_start + (dist_from_camera + noise * INSIDE_STEP_SIZE) * ray_direction;

        // If we are higher than the clouds or lower than the clouds, don't compute clouds
        if(current_position.z > CLOUD_LAYER_HEIGHTS.w + CLOUD_LAYER_THICKNESS && ray_direction.z > 0.0f) {
            //backdrop = vec4(1.0, 0.0, 0.0, 1.0);
            dist_from_camera = DRAW_DISTANCE;
            break;
        }
        if(current_position.z < CLOUD_LAYER_HEIGHTS.x - CLOUD_UNDERHANG && ray_direction.z < 0.0f) {
            //backdrop = vec4(0.0, 1.0, 0.0, 1.0);
            dist_from_camera = DRAW_DISTANCE;
            break;
        }

        float cloud_density_map = sampleCloudMapShape(cloud_map, current_position);

        if(cloud_density_map > 0.0f) {
            if(steps_outside_cloud != 0) {
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

        if(steps_outside_cloud <= STEP_OUTSIDE_RATIO && cloud_density_map > 0.0f) {
            float density_here = cloud_density_map;

            // We only need to sample the detailed cloud texture if
            // we are close and can see it in lots of detail.
            if(dist_from_camera < DRAW_DISTANCE / 3.0f) {
                density_here = addNoiseToDensity(current_position, density_here, CLOUD_NOISE_OCTAVES);
            }

            density_here = smoothstep(0.0f, 0.2f, density_here);

            density_here = max(density_here, 0.0f);
            float material_here = density_here * step_size;
            materialTowardsCamera += material_here;

            float materialTowardsSun = computeDensityTowardsSun(cloud_map, current_position, density_here);

            vec3 lightFromSunAtParticle = transmission(SUN_LIGHT * SUN_INTENSITY, materialTowardsSun);

            float angleToSun = dot(ray_direction, LIGHT_DIRECTION);

            vec3 lightAtParticle = lightFromSunAtParticle;
            vec3 lightScatteringTowardsCamera = lightScattering(lightAtParticle * material_here, angleToSun, materialTowardsCamera);
            vec3 lightReachingCamera = transmission(lightScatteringTowardsCamera, materialTowardsCamera);
            accumulation.rgb += lightReachingCamera;
        }

        if(materialTowardsCamera * CLOUD_DENSITY_SCALE > 4.0f) {
            break;
        }

        dist_from_camera += step_size;
        if(dist_from_camera > max_distance) {
            break;
        }
    }

    //float alpha = 1.0f - beer(materialTowardsCamera * (1.0f - BASE_TRANSMISSION));

    vec2 datavec = vec2(materialTowardsCamera, max_distance);
    uint data = packHalf2x16(datavec);
    float d = uintBitsToFloat(data);



    FragColor = vec4(
       
        accumulation,
        d
        
    );
}
