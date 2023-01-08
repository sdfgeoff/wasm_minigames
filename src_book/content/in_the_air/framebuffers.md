# In The Air Framebuffers

## Switching to Glow (and png's)
As with our original plan we need to create some framebuffers. Unlike with textures/
meshes, a framebuffer depends upon the textures, so first the textures (that the
framebuffers will write to) need to be created, then the framebuffers can be
created.

Note that `load_textures` also creates the empty textures for the framebuffers.

```rust
let textures = load_textures(&gl).expect("Failed to load textures");
let framebuffers = load_framebuffers(&gl, &textures).expect("Failed to load Fraimbuffers");

let renderer = RendererState {
    resolution: (canvas.client_width(), canvas.client_height()),
    pixels_per_centimeter: window().unwrap().device_pixel_ratio(),
    meshes: load_meshes(&gl).expect("Failed to load meshes"),
    shaders: load_shaders(&gl).expect("Failed to load shaders"),
    textures: textures,
    framebuffers: framebuffers,
};
```

The next question is: how do we test it? Well, why not set up the whole pipe?

Let's render something into the gbuffer, do something simple inside the lighting pass,
and splat that onto the screen at the end. We don't have a camera yet, so
we'll just splat a texture into the gbuffer.

So we need to:

 1. Bind the g-buffer
 2. Render some geometry
 3. Bind the dispay buffer
 4. Render a shader that samples the Gbuffer
 5. Bind the screen (A buffer of `None`)
 6. Render to the screen


<canvas id="in_the_air/framebuffers"></canvas>
