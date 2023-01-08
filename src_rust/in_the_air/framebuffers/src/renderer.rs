use super::framebuffer::{
    bind_texture_to_framebuffer_color, ColorAttachment, FrameBuffer, FrameBufferError,
};
use super::shader_program::{ShaderProgram, ShaderProgramError};
use super::texture::{
    EdgeMode, InterpolationMode, Texture, TextureConfig, TextureError, TexturePixelFormat,
};
use glow::{Context, HasContext};

use super::resources::StaticResources;

pub struct ShaderPrograms {
    model: ShaderProgram,
    volume_and_light: ShaderProgram,
    passthrough: ShaderProgram,
}

pub struct Textures {
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
    pub shader_programs: ShaderPrograms,
    pub textures: Textures,
    pub framebuffers: FrameBuffers,

    pub static_resources: StaticResources,
}

pub struct WorldState {
    pub time: f32,
}

pub fn load_shader_programs(
    gl: &Context,
    static_resources: &StaticResources,
) -> Result<ShaderPrograms, ShaderProgramError> {
    Ok(ShaderPrograms {
        model: ShaderProgram::new(
            gl,
            &static_resources.vertex_shaders.model_shader,
            &static_resources.fragment_shaders.model_shader,
            vec![],
        )?,
        volume_and_light: ShaderProgram::new(
            gl,
            &static_resources.vertex_shaders.full_screen_quad,
            &static_resources.fragment_shaders.volume_and_light,
            vec![
                "buffer_color".to_string(),
                "buffer_material".to_string(),
                "buffer_geometry".to_string(),
            ],
        )?,
        passthrough: ShaderProgram::new(
            gl,
            &static_resources.vertex_shaders.full_screen_quad,
            &static_resources.fragment_shaders.passthrough,
            vec!["input_texture".to_string()],
        )?,
    })
}

pub fn load_textures(gl: &Context, screen_resolution: &[i32; 2]) -> Result<Textures, TextureError> {
    let buffer_color = Texture::create_render_target(
        gl,
        TextureConfig {
            generate_mipmap: false,
            mag_interpolation: InterpolationMode::Nearest,
            min_interpolation: InterpolationMode::Nearest,
            edge_behaviour: EdgeMode::ClampToEdge,
        },
        TexturePixelFormat::RGBA8,
    )?;
    buffer_color.resize_render_target(gl, screen_resolution);

    let buffer_material = Texture::create_render_target(
        gl,
        TextureConfig {
            generate_mipmap: false,
            mag_interpolation: InterpolationMode::Nearest,
            min_interpolation: InterpolationMode::Nearest,
            edge_behaviour: EdgeMode::ClampToEdge,
        },
        TexturePixelFormat::RGBA8,
    )?;
    buffer_material.resize_render_target(gl, screen_resolution);

    let buffer_geometry = Texture::create_render_target(
        gl,
        TextureConfig {
            generate_mipmap: false,
            mag_interpolation: InterpolationMode::Nearest,
            min_interpolation: InterpolationMode::Nearest,
            edge_behaviour: EdgeMode::ClampToEdge,
        },
        TexturePixelFormat::RGBA16F,
    )?;
    buffer_geometry.resize_render_target(gl, screen_resolution);

    let buffer_display = Texture::create_render_target(
        gl,
        TextureConfig {
            generate_mipmap: false,
            mag_interpolation: InterpolationMode::Linear,
            min_interpolation: InterpolationMode::Linear,
            edge_behaviour: EdgeMode::ClampToEdge,
        },
        TexturePixelFormat::RGBA16F,
    )?;
    buffer_geometry.resize_render_target(gl, screen_resolution);

    Ok(Textures {
        buffer_color,
        buffer_material,
        buffer_geometry,

        buffer_display,
    })
}

pub fn load_framebuffers(
    gl: &Context,
    textures: &Textures,
) -> Result<FrameBuffers, FrameBufferError> {
    let gbuffer = FrameBuffer::new(gl)?;

    bind_texture_to_framebuffer_color(
        gl,
        &gbuffer,
        &textures.buffer_color,
        ColorAttachment::Attachment0,
    );
    bind_texture_to_framebuffer_color(
        gl,
        &gbuffer,
        &textures.buffer_geometry,
        ColorAttachment::Attachment1,
    );
    bind_texture_to_framebuffer_color(
        gl,
        &gbuffer,
        &textures.buffer_material,
        ColorAttachment::Attachment2,
    );

    unsafe {
        gl.draw_buffers(&[
            glow::COLOR_ATTACHMENT0,
            glow::COLOR_ATTACHMENT1,
            glow::COLOR_ATTACHMENT2,
        ]);
    }

    let display_buffer = FrameBuffer::new(gl)?;
    bind_texture_to_framebuffer_color(
        gl,
        &display_buffer,
        &textures.buffer_display,
        ColorAttachment::Attachment0,
    );
    unsafe {
        gl.draw_buffers(&[glow::COLOR_ATTACHMENT0]);
    }
    Ok(FrameBuffers {
        gbuffer: gbuffer,
        display_buffer: display_buffer,
    })
}

fn render_gbuffer(gl: &Context, renderer_state: &RendererState, _world_state: &WorldState) {
    // Render Opaque geometry to the G-buffer
    renderer_state.framebuffers.gbuffer.bind(gl);

    unsafe {
        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    renderer_state.shader_programs.model.bind(gl);

    renderer_state.static_resources.meshes.quad_quad.bind(
        gl,
        renderer_state.shader_programs.model.attrib_vertex_positions,
    );
    renderer_state.static_resources.meshes.quad_quad.render(gl);
}

fn render_volume_and_lighting(gl: &Context, renderer_state: &RendererState, _world_state: &WorldState) {
    // Render our GBuffer to the Display Buffer
    renderer_state.framebuffers.display_buffer.bind(gl);

    unsafe {
        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    renderer_state.shader_programs.volume_and_light.bind(gl);

    renderer_state.static_resources.meshes.quad_quad.bind(
        gl,
        renderer_state
            .shader_programs
            .volume_and_light
            .attrib_vertex_positions,
    );

    renderer_state.textures.buffer_color.bind_to_uniform(
        gl,
        0,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_color"),
    );
    renderer_state.textures.buffer_geometry.bind_to_uniform(
        gl,
        1,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_geometry"),
    );
    renderer_state.textures.buffer_material.bind_to_uniform(
        gl,
        2,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_material"),
    );

    renderer_state.static_resources.meshes.quad_quad.render(gl);
}



pub fn render(gl: &Context, renderer_state: &RendererState, world_state: &WorldState) {
    render_gbuffer(gl, renderer_state, world_state);
    render_volume_and_lighting(gl, renderer_state, world_state);

    {
        // Forward the display buffer to the screen
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            gl.viewport(
                0,
                0,
                renderer_state.resolution[0],
                renderer_state.resolution[1],
            );

            gl.clear_color(0.0, 0.0, 0.0, 0.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        renderer_state.shader_programs.passthrough.bind(gl);

        renderer_state.textures.buffer_display.bind_to_uniform(
            gl,
            0,
            renderer_state
                .shader_programs
                .passthrough
                .uniforms
                .get("input_texture"),
        );

        renderer_state.static_resources.meshes.quad_quad.bind(
            gl,
            renderer_state
                .shader_programs
                .passthrough
                .attrib_vertex_positions,
        );
        renderer_state.static_resources.meshes.quad_quad.render(gl);
    }
}

/// Configures the resolution of all of the textures used in the deferred geometry pipeline.
/// This ensure they match up with each other and the outside world.
pub fn resize_buffers(gl: &Context, renderer_state: &RendererState, resolution: &[i32; 2]) {
    renderer_state
        .textures
        .buffer_color
        .resize_render_target(gl, resolution);
    renderer_state
        .textures
        .buffer_material
        .resize_render_target(gl, resolution);
    renderer_state
        .textures
        .buffer_geometry
        .resize_render_target(gl, resolution);

    renderer_state
        .textures
        .buffer_display
        .resize_render_target(gl, resolution);
}
