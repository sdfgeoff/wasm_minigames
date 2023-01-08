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

Let's render a cube into the gbuffer, do something simple inside the lighting pass,
and splat that onto the screen at the end.


<canvas id="in_the_air/framebuffers"></canvas>
