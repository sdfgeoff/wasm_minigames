#version 300 es

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

// The color buffer contains RGB albedo values
uniform sampler2D buffer_color;

// The geometry buffer contains the normal and distance from camera
uniform sampler2D buffer_geometry;

// The material buffer contains the metallic (r channel) and the roughness (g channel)
uniform sampler2D buffer_material;

uniform sampler2D cloud_map;

uniform mat4 camera_to_screen;
uniform mat4 camera_to_world;
uniform mat4 world_to_camera;

const vec3 LIGHT_DIRECTION = vec3(0.0, 0.0, 1.0);

const float WORLD_SCALE = 0.05; // Scale all the cloud parameters by this amount

const float CLOUD_MAP_EXTENT = 128.0 * 60.0 * WORLD_SCALE; // 128 pixels, 60 meters per pixel
const vec4 CLOUD_LAYER_HEIGHTS = vec4(0.0, 1.0, 2.0, 4.0) * 1500.0 * WORLD_SCALE;
const float CLOUD_LAYER_THICKNESS = 2650.0 * WORLD_SCALE; // If this is bigger than the distance between the gap between CLOUD_LAYER_HEIGHTS then the clouds can overlap
const float CLOUD_UNDERHANG = CLOUD_LAYER_THICKNESS * 0.5; // How much the cloud layer extends below the layer height
const vec4 CLOUD_DENSITY_MAP_OFFSET = vec4(0.1); // Constant addition to the density of each layer
const float CLOUD_DENSITY_SCALE = 0.02;

float sample_volume_density(vec3 point) {
    vec4 map_sample = (textureLod(cloud_map, point.rg / CLOUD_MAP_EXTENT, 0.0) - 0.5) * 2.0;

    vec4 layer_density = map_sample + CLOUD_DENSITY_MAP_OFFSET;
    vec4 layer_centerline = CLOUD_LAYER_HEIGHTS + (CLOUD_LAYER_THICKNESS - CLOUD_UNDERHANG) * layer_density;
    vec4 layer_thickness = max(CLOUD_LAYER_THICKNESS * layer_density, 0.0);
    vec4 distance_to_centerline = abs(point.z - layer_centerline);
    vec4 distance_to_surface = distance_to_centerline - layer_thickness;
    vec4 distance_to_layer = distance_to_surface;

    float distance_to_cloud = min(min(min(distance_to_layer.x, distance_to_layer.y), distance_to_layer.z), distance_to_layer.w);

    float density = -distance_to_cloud;
    return density * CLOUD_DENSITY_SCALE;
}

vec4 light_surface(vec4 color, vec4 geometry, vec4 material) {
    vec3 normal = normalize(geometry.xyz);

    float diffuse = max(dot(normal, LIGHT_DIRECTION), 0.0);

    vec3 view_direction = normalize(camera_to_world[3].xyz - geometry.xyz);
    vec3 half_vector = normalize(LIGHT_DIRECTION + view_direction);
    float specular = pow(max(dot(normal, half_vector), 0.0), (material.g + 1.0) * 10.0);

    return vec4(diffuse * color.rgb + specular * material.r, 1.0);
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

    for(int i = 0; i < 100; i++) {
        float step_size = 5.0;
        vec3 p1 = ray_start + ray_direction * dist_from_camera;

        float density = sample_volume_density(p1);
        float absorbtion = min(max(density * step_size, 0.0), 1.0);

        if(dist_from_camera > opaque_distance_from_camera) {
            // We've hit something opaque, so light the surface and call
            // it done.

            vec4 color = texture(buffer_color, uv);
            vec4 material = texture(buffer_material, uv);
            outCol = alphaOver(outCol, light_surface(color, geometry, material));

            break;
        }

        vec4 color = vec4(0.2, 0.2, 0.3, absorbtion);
        outCol = alphaOver(outCol, color);

        if(outCol.a > 0.99) {
            // Fully opaque so anything behind this isn't visible anyway
            break;
        }

        if(dist_from_camera > 5000.0) {
            // We've rendered far enough away
            break;
        }

        dist_from_camera += step_size;// * max(distance_to_cloud, 1.0);
    }

    // Put in a black background:
    outCol = alphaOver(outCol, vec4(0.0, 0.0, 0.0, 1.0));

    FragColor = outCol;
}