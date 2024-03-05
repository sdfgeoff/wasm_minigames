#version 300 es

precision highp float;
precision highp int;
precision mediump sampler3D;

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



float beerPowder(float material_amount) {
    return pow(E, -material_amount) - pow(E, -material_amount * material_amount);
}

float beer(float material_amount) {
    return pow(E, -material_amount);
}


vec3 transmission(vec3 light, float material_amount) {
    return beer(material_amount * (1.0f - BASE_TRANSMISSION)) * light;
}


vec4 alphaOver(vec4 top, vec4 bottom) {
    float A1 = bottom.a * (1.0 - top.a);

    float A0 = top.a + A1;
    return vec4((top.rgb * top.a + bottom.rgb * A1) / A0, A0);
}


float hash14(vec4 p4)
{
	p4 = fract(p4  * vec4(.1031, .1030, .0973, .1099));
    p4 += dot(p4, p4.wzxy+33.33);
    return fract((p4.x + p4.y) * (p4.z + p4.w));
}




float sampleCloudMapShape(sampler2D cloud_map, vec3 point) {
    if(point.z > CLOUD_LAYER_HEIGHTS.w + CLOUD_LAYER_THICKNESS || point.z < CLOUD_LAYER_HEIGHTS.x - CLOUD_UNDERHANG) {
        return -100.0f;
    }
    vec4 map_sample = (textureLod(cloud_map, point.rg / CLOUD_MAP_EXTENT, 0.0f) - 0.5f) * 2.0f;

    vec4 layer_density = map_sample;
    vec4 layer_centerline = CLOUD_LAYER_HEIGHTS + (CLOUD_LAYER_THICKNESS - CLOUD_UNDERHANG) * layer_density;
    vec4 layer_thickness = max(CLOUD_LAYER_THICKNESS * layer_density, 0.0f);
    vec4 distance_to_centerline = abs(point.z - layer_centerline);
    vec4 distance_to_surface = distance_to_centerline - layer_thickness;
    vec4 distance_to_layer = distance_to_surface;

    float distance_to_cloud = min(min(min(distance_to_layer.x, distance_to_layer.y), distance_to_layer.z), distance_to_layer.w);

    float density = -distance_to_cloud;
    return density * CLOUD_DENSITY_SCALE;
}

float computeDensityTowardsSun(sampler2D cloud_map, vec3 current_position, float density_here) {
    float density_sunwards = max(density_here, 0.0f);
    density_sunwards += max(0.0f, sampleCloudMapShape(cloud_map, current_position + LIGHT_DIRECTION * 1.0f)) * 60.0f * WORLD_SCALE;
    density_sunwards += max(0.0f, sampleCloudMapShape(cloud_map, current_position + LIGHT_DIRECTION * 4.0f)) * 240.0f * WORLD_SCALE;

    return density_sunwards;
}