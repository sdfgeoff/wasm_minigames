# In The Air Volume Rendering

## Fly cam
Let's quickly build a flycam system. We need to alter the camera
transfrorm based on keyboard input. We'll use the keyup and keydown
events to maintain an internal state for each key. This is
not hard to do - just a bunch of boilerplate. Fortunately this is something
github copilot excells at, so isn't too hard.

Then we can make it modify the camera transform.

## Volume Rendering Time
What now? Time to render some volumetrics inside our volumetrics and
lighting shader.

There are three main parts to this volume rendering shader:

1. A raymarcher 
    - Use as few samples as possible to render large distances
    - Merging our volume rendering with the surface rendering

2. Rendering clouds
    - The "shape" or "density function" of the clouds
    - The lighting of the volume

3. Lighting the surface
    - How much light hits the surface of an object 
    - How do the material properties influence the lighting?

Let's get the complete pipe going with all these major sections, and then
we can go through one by one and make them better.

### Density map for clouds
First up let's create a density function for our clouds
```
float sample_volume_density(vec3 point) {
    return (20.0 - length(mod(point + 25.0, 50.0) - 25.0)) / 10.0;
}
```
Nothing too complex here, just standard signed-distance-field stuff, but
in our case a function that for a point in space returns how dense the clouds
are. 

It's using domain repetition (the `mod` function) to create an infinite
field of 20 unit radius spheres spaced 50 units apart. The density of each
sphere reaches `2.0` in the middle.

### Lighting a Surface
How do we light a surface? Uhm, let's ignore PBR for now and do a good old
fashioned diffuse-specular model:
```
vec4 light_surface(vec4 color, vec4 geometry, vec4 material) {
    vec3 normal = normalize(geometry.xyz);

    float diffuse = max(dot(normal, LIGHT_DIRECTION), 0.0);

    vec3 view_direction = normalize(camera_to_world[3].xyz - geometry.xyz);
    vec3 half_vector = normalize(LIGHT_DIRECTION + view_direction);
    float specular = pow(max(dot(normal, half_vector), 0.0), (material.g + 1.0) * 10.0);
    
    return vec4(diffuse * color.rgb + specular * material.r, 1.0);
}
```

### Raymarcher time
So, let's start at the camera and sample the density as we move forwards.
If we have hit the surface of an opaque object, light it. If not, continue
until either we run out of steps or go fully opaque. 

This is implemented as:
```glsl

vec4 alphaOver(vec4 top, vec4 bottom) {
    float A1 = bottom.a * (1.0 - top.a);
    
    float A0 = top.a + A1;
    return vec4(
        (top.rgb * top.a + bottom.rgb * A1) / A0,
        A0
    );
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
    vec4 ray_direction_world = camera_to_world * ray_direction_camera;
    
    vec3 ray_start = camera_to_world[3].xyz;
    vec3 ray_direction = normalize(ray_direction_world.xyz);

    
    float dist_from_camera = 0.0;
    vec4 outCol = vec4(0.0, 0.0, 0.0, 0.001); // This small starting opacity prevents a div-zero error in alpha compositing

    for (int i = 0; i < 100; i++) {
        float step_size = 2.0;
        vec3 p1 = ray_start + ray_direction * dist_from_camera;

        float density = sample_volume_density(p1)* 0.1;
        float absorbtion = min(max(density * step_size, 0.0), 1.0);

        if (dist_from_camera > opaque_distance_from_camera) {
            // We've hit something opaque, so light the surface and call
            // it done.

            vec4 color = texture(buffer_color, uv);
            vec4 material = texture(buffer_material, uv);
            outCol = alphaOver(outCol, light_surface(color, geometry, material));

            break;
        }
        

        vec4 color = vec4(0.2, 0.2, 0.3, absorbtion);
        outCol = alphaOver(outCol, color);

        if (outCol.a > 0.99) {
            // Fully opaque so anything behind this isn't visible anyway
            break;
        }

        dist_from_camera += step_size;
    }

    // Put in a black background:
    outCol = alphaOver(outCol, vec4(0.0, 0.0, 0.0, 1.0));

    FragColor = outCol;
}
```

What does it all look like (use WSADQERF and arrow keys to move the camera):

<canvas id="in_the_air/rendering_volumes"></canvas>

Not too bad. Definitely not very modern looking though. The opaque surfaces look
quite dull (modern PBR is way better), the volumes have no lighting, and there
is obvious banding coming from the raymarcher. But that's OK, we can work through
each one by one to improve it. 