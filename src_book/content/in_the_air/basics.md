# In The Air Basics

## Switching to Glow (and png's)
So as mentioned in the intro, we're switching to use `glow` for openGL. Mostly everything is the same, but we now have to load images ourselves and feed the pixel data across to the GPU. 

PNG is a surprisingly complex format and is capable of supporting a whole bunch of pixel formats. So we have to map from a PNG format into an openGLformat. We can do a simple match on the output from the PNG reader:

```rust
let tex_format = match reader.output_color_type() {
    (ColorType::Rgb, BitDepth::Eight) => TexturePixelFormat::RGB8,
    (ColorType::Rgb, BitDepth::Sixteen) => TexturePixelFormat::RGBA16UI,
    (ColorType::Rgba, BitDepth::Eight) => TexturePixelFormat::RGBA8,
    (ColorType::Rgba, BitDepth::Sixteen) => TexturePixelFormat::RGBA16UI,
    (ColorType::Grayscale, BitDepth::Eight) => TexturePixelFormat::R8,
    (ColorType::Grayscale, BitDepth::Sixteen) => TexturePixelFormat::R16UI,
    (_, _) => unimplemented!("Unsupported PNG Pixel Type"),
};
```

What is this `TexturePixelFormat`? It's a flippin massive enum that I made that encodes all the openGL pixel formats and has functions for turning them into openGL constants:
```rust
pub enum TexturePixelFormat {
    R8,
    R8_SNORM,
    R16F,
    R32F,
    R8UI,
    R8I,
    R16UI,
    R16I,
    ... // 50 odd options
    

<snip>

impl TexturePixelFormat {
    pub fn to_sized_internal_format(&self) -> u32 {
        match self {
            Self::R8 => glow::R8,
            Self::R8_SNORM => glow::R8_SNORM,
            Self::R16F => glow::R16F,
            Self::R32F => glow::R32F,
            Self::R8UI => glow::R8UI,
            Self::R8I => glow::R8I,

    <snip>

    pub fn to_format(&self) -> u32 {
        match self {
            Self::R8 => glow::RED,
            Self::R8_SNORM => glow::RED,
            Self::R16F => glow::RED,
            Self::R32F => glow::RED,
            Self::R8UI => glow::RED_INTEGER,
            Self::R8I => glow::RED_INTEGER,
            Self::R16UI => glow::RED_INTEGER,

    <snip>

    pub fn to_type(&self) -> u32 {
        match self {
            Self::R8 => glow::UNSIGNED_BYTE,
            Self::R8_SNORM => glow::BYTE,
            Self::R16F => glow::HALF_FLOAT,
            Self::R32F => glow::FLOAT,
            Self::R8UI => glow::UNSIGNED_BYTE,
            Self::R8I => glow::BYTE,
            Self::R16UI => glow::UNSIGNED_SHORT,
            Self::R16I => glow::SHORT,
    
    <snip>

    pub fn to_channel_count(&self) -> usize {
        match self {
            Self::R8 => 1,
            Self::R8_SNORM => 1,
            Self::R16F => 1,
            Self::R32F => 1,
            Self::R8UI => 1,
            Self::R8I => 1,
            Self::R16UI => 1,

```

This means that when we come to load the data into the texture, we can easily communicate to opengl what is in the buffer:
```rust
gl.tex_storage_2d(
    glow::TEXTURE_2D,
    levels,
    tex_format.to_sized_internal_format(),
    info.width as i32,
    info.height as i32,
);

gl.tex_sub_image_2d(
    glow::TEXTURE_2D,
    0,
    0,
    0,
    info.width as i32,
    info.height as i32,
    tex_format.to_format(),
    tex_format.to_type(),
    glow::PixelUnpackData::Slice(image_pixels),
);
```

I did similar things with edge wrapping, and combined it all into a utility function:
```rust
pub struct TextureConfig {
    pub generate_mipmap: bool,
    pub mag_interpolation: InterpolationMode,
    pub min_interpolation: InterpolationMode,
    pub edge_behaviour: EdgeMode,
}

pub fn load_from_png(
    gl: &Context,
    data: &[u8],
    config: &TextureConfig,
) -> Result<Texture, TextureError> {
```
Now we can load our textures using `include_bytes!()` and some small configuration.

## Structuring our program and Resource Management
We are going to have a bunch of shaders and a bunch of meshes and textures. We also have various gameplay logic. Unlike with Swoop where I created explicit structs for each sprite, I'd like it to be a bit more functional. There'll be a function for rendering a specific object, but all the parameters must be passed into that function.

One of those parameters is the resources - the record of what is on the GPU. This is a struct that looks like:

```rust
pub struct Meshes {
    quad: Mesh,
}

pub struct Shaders {
    test_shader: Shader,
}

pub struct Textures {
    test_texture1: Texture,
    test_texture2: Texture,
}

pub struct RendererState {
    pub resolution: (i32, i32),
    pub pixels_per_centimeter: f64,
    pub meshes: Meshes,
    pub shaders: Shaders,
    pub textures: Textures,
}
```

A function that renders an object now looks like:
```rust
renderer_state.shaders.test_shader.bind(gl);
renderer_state.textures.test_texture1.bind_to_uniform(
    gl,
    0,
    renderer_state
        .shaders
        .test_shader
        .uniforms
        .get("image_texture_1"),
);
renderer_state.textures.test_texture2.bind_to_uniform(
    gl,
    1,
    renderer_state
        .shaders
        .test_shader
        .uniforms
        .get("image_texture_2"),
);
renderer_state.meshes.quad.bind(
    gl,
    renderer_state.shaders.test_shader.attrib_vertex_positions,
);
renderer_state.meshes.quad.render(gl);
```

I think this is tidy enough, and it allows sharing tetures and shader programs bewteen different in-game entities.

## All Done?
Yeah, all switched to `glow`. There are quite a few other minor changes - I pulled in code for generating mipmaps etc. from other projects I've worked on over the past few months. But here it is, looking exactly like the [Binding Textures](../basics/binding_textures.md) page, but now in `glow` and with our new resource management.

<canvas id="in_the_air/basics"></canvas>

Oh yeah, it's flipped vertically. WebGL had `gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);`, but OpenGLES doesn't have the `UNPACK_FLIP` texture storage option. Oh well, I'll just flip my textures manually on disk 😀.