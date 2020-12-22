# Basic Deferred

The idea behind a deferred renderer is that you wait until later to
do the lighting - so for each object in the scene you render out the
surface information (position, normals, material) to a framebuffer, and
then later you can go through and render the lighting.

## Setting up a framebuffer
So, how do we do this? We need to create a framebuffer. A framebuffer
is a set of textures that you can render to instead of the screen. You 
can render to it using `gl.bind_framebuffer(GL::FRAMEBUFFER, 
Some(buffer))` before rendering the geometry, and then calling 
`gl.bind_framebuffer(GL::FRAMEBUFFER, None)` to render to the 
viewport/screen.

So let's create a framebuffer:
```rust
let buffer = gl.create_framebuffer();
gl.bind_framebuffer(GL::FRAMEBUFFER, buffer.as_ref());
```

Now that the framebuffer is bound, we can create textures "inside" it:
```rust
gl.active_texture(GL::TEXTURE0);

// TODO update resolution!
let width = resolution.0 as i32; //gl.drawing_buffer_width();
let height = resolution.1 as i32; //gl.drawing_buffer_height();

let normal_depth_target = gl.create_texture();
gl.bind_texture(GL::TEXTURE_2D, normal_depth_target.as_ref());
gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 0);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
gl.tex_storage_2d(GL::TEXTURE_2D, 1, GL::RGBA16F, width, height);
gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, normal_depth_target.as_ref(), 0);

let albedo_target = gl.create_texture();
gl.bind_texture(GL::TEXTURE_2D, albedo_target.as_ref());
gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 0);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
gl.tex_storage_2d(GL::TEXTURE_2D, 1, GL::RGBA8, width, height);
gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT1, GL::TEXTURE_2D, albedo_target.as_ref(), 0);

let depth_target = gl.create_texture();
gl.bind_texture(GL::TEXTURE_2D, depth_target.as_ref());
gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 0);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
gl.tex_storage_2d(GL::TEXTURE_2D, 1, GL::DEPTH_COMPONENT16, width, height);
gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::DEPTH_ATTACHMENT, GL::TEXTURE_2D, depth_target.as_ref(), 0);
```

This isn't quite enough, we need to tell openGL how to connect them to our shader.
So in rust we put them in an array:
```rust
let buff = Array::new();
buff.set(0, GL::COLOR_ATTACHMENT0.into());
buff.set(1, GL::COLOR_ATTACHMENT1.into());
gl.draw_buffers(&buff);
```

And in our shader, we specify the position in the array:
```glsl
layout(location=0) out vec4 normal_depth;
layout(location=1) out vec4 albedo;
```

Now all we have to do is call `gl.bind_framebuffer(GL::FRAMEBUFFER, 
self.buffer.as_ref())` before rendering some geometry.

## But what do we render?

But what do we put in our framebuffer, and how do we turn that into a
composited lit image?

Well, our model shader can be extremely simple:
```glsl
#version 300 es

precision mediump float;
in vec3 screen_nor;
in vec3 world_nor;
in vec3 screen_pos;
in vec2 uv0;

layout(location=0) out vec4 normal_depth;
layout(location=1) out vec4 albedo;

in mat4 camera_to_world;

uniform sampler2D image_albedo;
uniform vec4 color;

void main() {
    albedo = color * texture(image_albedo, uv0);
    normal_depth = vec4(world_nor, 1.0);
}
```

We only sample the color texture and then write them (and the vertex 
normals) into the output textures. Nice and simple.

Our lighting shader takes the place of all the matcap stuff from the
previous 3D rendering:

```glsl
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
    
    vec3 lighting = sample_background(normal_depth.xyz, 3.0);
    
    vec3 out_col = lighting * albedo.rgb;
    
    FragColor.rgb = out_col;
    FragColor.a = 1.0;
}
```

We're still not using actual lights here, but we're using the lighting 
information from a texture, so (to me) this counts as a deferred 
renderer.


There was a bit more finangling than this (the framebuffer is in it's
own struct, as is the lighting pass), but that's the basics of it.


I've disabled the cockpit glass for now as it requires transparency -
something deferred renderers struggle with. Will I re-enable it? Maybe.
You'll also notice a lot more aliasing artifaces - another disadvantage
of deferred rendering. But the result so far:

<canvas id="pilot_basic_deferred"></canvas>



