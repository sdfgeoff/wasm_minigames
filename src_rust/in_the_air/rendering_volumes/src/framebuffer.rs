use super::texture::Texture;
use glow::{Context, HasContext};

pub struct FrameBuffer {
    pub framebuffer: glow::Framebuffer,
}

#[derive(Debug)]
pub enum FrameBufferError {
    CreateError(String),
}

impl FrameBuffer {
    pub fn new(gl: &Context) -> Result<Self, FrameBufferError> {
        let framebuffer = unsafe {
            gl.create_framebuffer()
                .map_err(FrameBufferError::CreateError)?
        };
        Ok(Self { framebuffer })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
        }
    }
}

pub fn bind_texture_to_framebuffer_color(
    gl: &Context,
    framebuffer: &FrameBuffer,
    texture: &Texture,
    attach: ColorAttachment,
) {
    framebuffer.bind(gl);
    unsafe {
        gl.active_texture(glow::TEXTURE0);

        gl.bind_texture(glow::TEXTURE_2D, Some(texture.tex));

        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            attach.to_gl_const(),
            glow::TEXTURE_2D,
            Some(texture.tex),
            0,
        );
    }
}

pub fn bind_texture_to_framebuffer_depth(
    gl: &Context,
    framebuffer: &FrameBuffer,
    texture: &Texture,
) {
    framebuffer.bind(gl);
    unsafe {
        gl.active_texture(glow::TEXTURE0);

        gl.bind_texture(glow::TEXTURE_2D, Some(texture.tex));

        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::DEPTH_ATTACHMENT,
            glow::TEXTURE_2D,
            Some(texture.tex),
            0,
        );
    }
}

#[allow(dead_code)]
pub enum ColorAttachment {
    Attachment0,
    Attachment1,
    Attachment2,
    Attachment3,
    Attachment4,
    // Can go up to 32
}

impl ColorAttachment {
    pub fn to_gl_const(&self) -> u32 {
        match self {
            Self::Attachment0 => glow::COLOR_ATTACHMENT0,
            Self::Attachment1 => glow::COLOR_ATTACHMENT1,
            Self::Attachment2 => glow::COLOR_ATTACHMENT2,
            Self::Attachment3 => glow::COLOR_ATTACHMENT3,
            Self::Attachment4 => glow::COLOR_ATTACHMENT4,
        }
    }
}
