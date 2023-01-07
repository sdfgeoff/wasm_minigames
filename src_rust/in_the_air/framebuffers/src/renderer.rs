use super::mesh::{Mesh, MeshError};
use super::shader::{Shader, ShaderError};
use super::texture::{Texture, TextureConfig, TextureError, InterpolationMode, TexturePixelFormat, EdgeMode};
use super::framebuffer::{FrameBuffer, FrameBufferError, bind_texture_to_framebuffer_color, ColorAttachment};
use glow::{Context, HasContext};

pub struct Meshes {
    quad: Mesh,
}

pub struct Shaders {
    test_shader: Shader,
}

pub struct Textures {
    test_texture1: Texture,
    test_texture2: Texture,

    buffer_color: Texture,
    buffer_material: Texture,
    buffer_geometry: Texture,

    buffer_display: Texture,
}

pub struct FrameBuffers {
    gbuffer: FrameBuffer,
    display_buffer: FrameBuffer,
}

pub struct RendererState {
    pub resolution: [i32; 2],
    pub pixels_per_centimeter: f64,
    pub meshes: Meshes,
    pub shaders: Shaders,
    pub textures: Textures,
    pub framebuffers: FrameBuffers,
}

pub struct WorldState {
    pub time: f32,
}

pub fn load_meshes(gl: &Context) -> Result<Meshes, MeshError> {
    let quad = Mesh::new(
        gl,
        &[-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
        &[0, 1, 2, 0, 2, 3],
    )?;

    Ok(Meshes { quad })
}

pub fn load_shaders(gl: &Context) -> Result<Shaders, ShaderError> {
    let test_shader = Shader::new(
        gl,
        include_str!("resources/FullScreenQuad.vert"),
        include_str!("resources/test.frag"),
        vec!["image_texture_1".to_string(), "image_texture_2".to_string()],
    )?;

    Ok(Shaders { test_shader })
}

pub fn load_textures(gl: &Context, screen_resolution: &[i32; 2]) -> Result<Textures, TextureError> {
    let test_texture1 = Texture::load_from_png(
        gl,
        include_bytes!("resources/test.png"),
        TextureConfig::default(),
    )?;
    let test_texture2 = Texture::load_from_png(
        gl,
        include_bytes!("resources/test2.png"),
        TextureConfig::default(),
    )?;


    let buffer_color = Texture::create_render_target(gl, TextureConfig {
        generate_mipmap: false,
        mag_interpolation: InterpolationMode::Nearest,
        min_interpolation: InterpolationMode::Nearest,
        edge_behaviour: EdgeMode::ClampToEdge,
    }, TexturePixelFormat::RGBA8)?;
    buffer_color.resize_render_target(gl, screen_resolution);

    let buffer_material = Texture::create_render_target(gl, TextureConfig {
        generate_mipmap: false,
        mag_interpolation: InterpolationMode::Nearest,
        min_interpolation: InterpolationMode::Nearest,
        edge_behaviour: EdgeMode::ClampToEdge,
    }, TexturePixelFormat::RGBA8)?;
    buffer_material.resize_render_target(gl, screen_resolution);

    let buffer_geometry = Texture::create_render_target(gl, TextureConfig {
        generate_mipmap: false,
        mag_interpolation: InterpolationMode::Nearest,
        min_interpolation: InterpolationMode::Nearest,
        edge_behaviour: EdgeMode::ClampToEdge,
    }, TexturePixelFormat::RGBA16F)?;
    buffer_geometry.resize_render_target(gl, screen_resolution);


    let buffer_display = Texture::create_render_target(gl, TextureConfig {
        generate_mipmap: false,
        mag_interpolation: InterpolationMode::Linear,
        min_interpolation: InterpolationMode::Linear,
        edge_behaviour: EdgeMode::ClampToEdge,
    }, TexturePixelFormat::RGBA16F)?;
    buffer_geometry.resize_render_target(gl, screen_resolution);



    Ok(Textures {
        test_texture1,
        test_texture2,

        buffer_color,
        buffer_material,
        buffer_geometry,

        buffer_display,
    })
}



pub fn load_framebuffers(gl: &Context, textures: &Textures) -> Result<FrameBuffers, FrameBufferError>  {
    let gbuffer = FrameBuffer::new(gl)?;

    bind_texture_to_framebuffer_color(gl, &gbuffer, &textures.buffer_color, ColorAttachment::Attachment0);
    bind_texture_to_framebuffer_color(gl, &gbuffer, &textures.buffer_geometry, ColorAttachment::Attachment1);
    bind_texture_to_framebuffer_color(gl, &gbuffer, &textures.buffer_material, ColorAttachment::Attachment2);

    let display_buffer = FrameBuffer::new(gl)?;
    bind_texture_to_framebuffer_color(gl, &display_buffer, &textures.buffer_display, ColorAttachment::Attachment0);

    Ok(FrameBuffers {
        gbuffer: gbuffer,
        display_buffer: display_buffer
    })
}

pub fn render(gl: &Context, renderer_state: &RendererState, world_state: &WorldState) {
    unsafe {

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);

        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

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
}


pub fn resize_buffers(gl: &Context, renderer_state: &RendererState, resolution: &[i32; 2]) {
    renderer_state.textures.buffer_color.resize_render_target(gl, resolution);
    renderer_state.textures.buffer_material.resize_render_target(gl, resolution);
    renderer_state.textures.buffer_geometry.resize_render_target(gl, resolution);

    renderer_state.textures.buffer_display.resize_render_target(gl, resolution);
}