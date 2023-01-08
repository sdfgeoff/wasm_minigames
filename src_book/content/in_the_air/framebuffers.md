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
and splat that onto the screen at the end. 

So we need to:

 1. Bind the g-buffer
 2. Render some geometry
 3. Bind the dispay buffer
 4. Render a shader that samples the Gbuffer
 5. Bind the screen (A buffer of `None`)
 6. Render to the screen


Binding the framebuffer is easy enough, a call to gl.bindFramebuffer does that
and means that any subsequent draw calls go into the buffer.

We don't have a camera or transforms yet, so
we'll just splat a texture into the gbuffer directly.
```frag
#version 300 es

precision mediump float;

uniform sampler2D albedo_texture;
uniform sampler2D metallic_roughness_texture;

in vec2 uv0;

layout(location=0) out vec4 buffer_color;
layout(location=1) out vec4 buffer_geometry;
layout(location=2) out vec4 buffer_material;

void main() {
    buffer_color = texture(albedo_texture, uv0);
    buffer_material = texture(metallic_roughness_texture, uv0);
    buffer_geometry = vec4(uv0, 1.0, 0.5);
}
```

I then spent two hours debugging a problem where the same texture was appearing in
both the color buffer and material buffer. Inspecting with  spector.js` browser
plugin revealed that both of the input textures were the same. After inspecting
the code for setting up textures, checking that the two textures were on
different texture units, I finally figured out that ... I was assigning the
same texture to both! Whoops.

The display shader splits the three buffers to each side, and a passthrough 
shader splats them onto the screen, and wth that...

<canvas id="in_the_air/framebuffers"></canvas>

