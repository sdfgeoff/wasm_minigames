# Faster Clouds

Currently the clouds really chug on my laptop. It's dropping frames at 640x480, and is ~10fps at 1080p. I reckon we can do better.


## Don't Render clouds if outside of cloud box or behind opaque geometry
While we have a render sphere for draw distance, there is also a limit to how high and how low the clouds can be. There is no reason why we should bother to raymarch when outside of these bounds. Similarly, if there is geometry we don't need to continue raymarching.

This can be checked via:
```glsl
        // If we are higher than the clouds or lower than the clouds, don't compute clouds
        if (current_position.z > CLOUD_LAYER_HEIGHTS.w + CLOUD_LAYER_THICKNESS && ray_direction.z > 0.0) {
            backdrop = vec4(1.0, 0.0, 0.0, 1.0);
            break;
        }
        if (current_position.z < CLOUD_LAYER_HEIGHTS.x - CLOUD_UNDERHANG && ray_direction.z < 0.0) {
            backdrop = vec4(0.0, 1.0, 0.0, 1.0);
            break;
        }
```

And a small refactor to combine both draw distance, sky and opaque geometry all together:

```glsl
    // Backdrop
    vec4 backdrop = vec4(0.0);
    vec4 geometry = texture(buffer_geometry, uv);

    if (geometry.w == 0.0) {
        backdrop = vec4(renderSky(ray_direction), DRAW_DISTANCE);
    } else {
        float opaque_distance_from_camera = geometry.w;
        vec4 color = texture(buffer_color, uv);
        vec4 material = texture(buffer_material, uv);
        float materialTowardsSun = computeDensityTowardsSun(ray_start + ray_direction * opaque_distance_from_camera, 0.0);
        vec3 lightFromSunAtParticle = transmission(
            SUN_LIGHT * SUN_INTENSITY,
            materialTowardsSun
        );
        backdrop = vec4(
            light_surface(color, geometry, material, lightFromSunAtParticle).rgb,
            opaque_distance_from_camera
        );
    }

<snip>

        if (dist_from_camera > backdrop.a) {
            backdrop = vec4(0.0, 1.0, 1.0, 1.0);
            break;
        }

```
You can see in each of those I'm writing a color for each exit condition.
That means I can visualize that those conditions are being hit correctly.

<img src="early_return_altitude.png" />


# Where else are we using lots of raymarch steps?
If we write the loop counter into a variable, we can draw this one too:

<img src="step_count.png" />

This tells us what parts of our scene are taking lots of computation.
As expected, the further you see, the more likely it is to run out of steps.
It'll run out of steps before it hits the render distance limit.

We could increase step size, but that would decrease quality. But maybe we can
decrease step size at long distance. The current cloud implementation does
a clever long-stride and then backtracks if it encounters a cloud. We can
ditch the back-tracking after a certain render distance to make better use of steps. I tried it, and it did something funky with coloring. I have no idea
if it's to do with the backtracking or some step-size-dependence in my lighting code.

# Split out the volumetrics to a low resolution pass
Since clouds are fluffy, we should be able to get away with rendering them at 1/2 resolution. FOr this we do need another framebuffer. Fortunately this is
easy enough now as we have all the code fairly well abstracted, and we've just
split our shader to compute the backdrop of opaque and composite things on top.

So after a bit of fillding:
<img src="outlines.png"/>

Ewwwww. Look at that outline around the cloud and around the vehicle. That's because it's half-resolution. Dang. Anywah, at least it is fast. I can hit 60FPS in fullscreen now. So I'll call that a win at least. But what can be done about the appearance? 

The issue is that the volume buffer is half-resolution, so around the edges of objects it will alias as sometimes it will get the voume buffer at the right depth, and sometimes it will get it at the wrong depth.

We need to do a depth-aware sample of the volume buffer to ensure that we occlude using the sample closest to the depth of the surface.

I couldn't think of a nice way to do this so I packed the depth into the alpha channel of the volume buffer and...

```glsl

vec4 sample_volume(in vec2 uv, out float depth) {
    vec4 raw = texture(volume_texture, uv);
    uint data_uint = floatBitsToUint(raw.a);
    vec2 data = unpackHalf2x16(data_uint);

    depth = data.y;
    float mat = data.x;

    return vec4(
        raw.rgb,
        mat
    );
}


void main() {
    vec2 uv = screen_pos.xy * 0.5 + 0.5;

    vec4 opaque = texture(lighting_texture, uv);
    float surfaceDepth = opaque.a;

    vec2 offset = 1.0 / resolution * 2.0;
    float depth1, depth2, depth3, depth4 = 0.0;
    vec4 v1 = sample_volume(uv + offset * vec2(-1, 0), depth1);
    vec4 v2 = sample_volume(uv + offset * vec2(1, 0), depth2);
    vec4 v3 = sample_volume(uv + offset * vec2(0, 1), depth3);
    vec4 v4 = sample_volume(uv + offset * vec2(0, -1), depth4);

    // Find out which volume sample that is nearest to the surfae depth
    vec4 deltas = abs(vec4(surfaceDepth) - vec4(depth1, depth2, depth3, depth4));
    float minDelta = min(min(min(deltas.x, deltas.y), deltas.z), deltas.w);

    vec4 volume = v1;
    if (minDelta == deltas.x) {
        volume = v1;
    } else if (minDelta == deltas.y) {
        volume = v2;
    } else if (minDelta == deltas.z) {
        volume = v3;
    } else if (minDelta == deltas.w) {
        volume = v4;
    }
    
    float materialTowardsCamera = volume.a;

    vec3 color = volume.rgb + beer(materialTowardsCamera * (1.0 - BASE_TRANSMISSION)) * opaque.rgb;

    FragColor = vec4(color.rgb, 1.0);
}

```
It's kinda verbose and branch-y, but it works and runs like butter at 1080p on my laptop.


Also, there's now heaps of dupicated shader code. Maybe I need a preprocessor
to make it easier to have common functions. Plain old string concatenation will work fine.

```rust
impl FragmentShaders {
    pub fn load(gl: &Context) -> Result<Self, ShaderError> {
        Ok(Self {
            model_shader: Shader::new(gl, ShaderType::Fragment, include_str!("model_shader.frag"))?,
            volume_and_light: Shader::new(
                gl,
                ShaderType::Fragment,
                &(include_str!("common.frag").to_owned() + include_str!("volume_and_light.frag")),
            )?,
            volume: Shader::new(
                gl,
                ShaderType::Fragment,
                &(include_str!("common.frag").to_owned() + include_str!("volume.frag")),
            )?,
            passthrough: Shader::new(
                gl,
                ShaderType::Fragment,
                &(include_str!("common.frag").to_owned() + include_str!("passthrough.frag")),
            )?,
        })
    }
}
```

My IDE now screams at me about undefined variables, but it all compiles and runs, so I don't really mind.



<canvas id="in_the_air/faster_clouds"></canvas>