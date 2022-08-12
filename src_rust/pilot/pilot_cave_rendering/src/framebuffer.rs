use js_sys::Array;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::texture::TextureError;

#[derive(Debug)]
pub struct GBuffer {
    pub normal_depth_target: Option<WebGlTexture>,
    pub albedo_target: Option<WebGlTexture>,
    pub depth_target: Option<WebGlTexture>,
    pub buffer: Option<WebGlFramebuffer>,
}

impl GBuffer {
    pub fn new(gl: &WebGl2RenderingContext, resolution: (u32, u32)) -> Result<Self, TextureError> {
        let buffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, buffer.as_ref());

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
        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            normal_depth_target.as_ref(),
            0,
        );

        let albedo_target = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, albedo_target.as_ref());
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 0);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_storage_2d(GL::TEXTURE_2D, 1, GL::RGBA8, width, height);
        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT1,
            GL::TEXTURE_2D,
            albedo_target.as_ref(),
            0,
        );

        let depth_target = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, depth_target.as_ref());
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 0);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_storage_2d(GL::TEXTURE_2D, 1, GL::DEPTH_COMPONENT16, width, height);
        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::TEXTURE_2D,
            depth_target.as_ref(),
            0,
        );

        let buff = Array::new();
        buff.set(0, GL::COLOR_ATTACHMENT0.into());
        buff.set(1, GL::COLOR_ATTACHMENT1.into());
        gl.draw_buffers(&buff);

        if normal_depth_target.is_none() || albedo_target.is_none() || depth_target.is_none() {
            return Err(TextureError::AllocateTextureError);
        }

        Ok(Self {
            normal_depth_target,
            albedo_target,
            depth_target,
            buffer,
        })
    }

    pub fn delete(&mut self, gl: &WebGl2RenderingContext) {
        gl.delete_framebuffer(self.buffer.take().as_ref());
        gl.delete_texture(self.normal_depth_target.take().as_ref());
        gl.delete_texture(self.depth_target.take().as_ref());
        gl.delete_texture(self.albedo_target.take().as_ref());
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, self.buffer.as_ref())
    }
}
