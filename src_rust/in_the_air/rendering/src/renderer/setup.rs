/**
 * Functinos to set up the renderer.
 */
use crate::framebuffer::{
    bind_texture_to_framebuffer_color, bind_texture_to_framebuffer_depth, ColorAttachment, FrameBuffer, FrameBufferError,
};
use crate::shader_program::{ShaderProgram, ShaderProgramError};
use crate::texture::{
    EdgeMode, InterpolationMode, Texture, TextureConfig, TextureError, TexturePixelFormat,
};
use glow::{Context, HasContext};


use super::{FrameBuffers, Textures, ShaderPrograms, StaticResources};

/// Combines fragment and vertex shaders from the static resources
/// into shader programs with known uniform names
pub fn load_shader_programs(
    gl: &Context,
    static_resources: &StaticResources,
) -> Result<ShaderPrograms, ShaderProgramError> {
    Ok(ShaderPrograms {
        model: ShaderProgram::new(
            gl,
            &static_resources.vertex_shaders.model_shader,
            &static_resources.fragment_shaders.model_shader,
            vec![
                "metallic_roughness_texture".to_string(),
                "albedo_texture".to_string(),

                "world_to_model".to_string(),
                "model_to_world".to_string(),
                "camera_to_screen".to_string(),

                "camera_to_world".to_string(),
                "world_to_camera".to_string(),
            ],
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

/// Not all textures can be statically defined. This loads some
/// the ones that are created at runtime such as framebuffer targets
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

    let buffer_depth = Texture::create_render_target(
        gl,
        TextureConfig {
            generate_mipmap: false,
            mag_interpolation: InterpolationMode::Nearest,
            min_interpolation: InterpolationMode::Nearest,
            edge_behaviour: EdgeMode::ClampToEdge,
        },
        TexturePixelFormat::DEPTH_COMPONENT16,
    )?;
    buffer_depth.resize_render_target(gl, screen_resolution);

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
        buffer_depth,

        buffer_display,
    })
}

/// The Frambuffers need to be set up with their render targets...
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
    bind_texture_to_framebuffer_depth(
        gl,
        &gbuffer,
        &textures.buffer_depth,
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