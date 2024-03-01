use glow::{Context, HasContext};
use png::{BitDepth, ColorType};

#[derive(Debug)]
pub enum TextureError {
    CreateTextureFailed(String),
}

#[allow(dead_code)]
pub enum InterpolationMode {
    Nearest,
    Linear,
    LinearMipMapLinear,
}

impl InterpolationMode {
    pub fn to_gl(&self) -> i32 {
        match self {
            Self::Nearest => glow::NEAREST as i32,
            Self::Linear => glow::LINEAR as i32,
            Self::LinearMipMapLinear => glow::LINEAR_MIPMAP_LINEAR as i32,
        }
    }
}

#[allow(dead_code)]
pub enum Dimension {
    D1,
    D2,
    D3(u32, u32, u32), // Width, height, depth
}

impl Dimension {
    pub fn to_gl(&self) -> u32 {
        match self {
            Self::D1 => glow::TEXTURE_1D,
            Self::D2 => glow::TEXTURE_2D,
            Self::D3(_, _, _) => glow::TEXTURE_3D,
        }
    }
}

#[allow(dead_code)]
pub enum EdgeMode {
    Repeat,
    Mirror,
    ClampToEdge,
}
impl EdgeMode {
    pub fn to_gl(&self) -> i32 {
        match self {
            Self::Repeat => glow::REPEAT as i32,
            Self::Mirror => glow::MIRRORED_REPEAT as i32,
            Self::ClampToEdge => glow::CLAMP_TO_EDGE as i32,
        }
    }
}

pub struct TextureConfig {
    pub generate_mipmap: bool,
    pub mag_interpolation: InterpolationMode,
    pub min_interpolation: InterpolationMode,
    pub edge_behaviour: EdgeMode,
    pub dimension: Dimension,
}

pub struct Texture {
    pub tex: glow::Texture,
    format: TexturePixelFormat,
    config: TextureConfig,
}

impl Default for TextureConfig {
    fn default() -> Self {
        return Self {
            generate_mipmap: true,
            mag_interpolation: InterpolationMode::Linear,
            min_interpolation: InterpolationMode::LinearMipMapLinear,
            edge_behaviour: EdgeMode::Repeat,
            dimension: Dimension::D2,
        };
    }
}

impl Texture {
    pub fn load_from_png(
        gl: &Context,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<Texture, TextureError> {
        let decoder = png::Decoder::new(data);
        let mut reader = decoder.read_info().unwrap();

        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An Animated PNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        let image_pixels = &buf[..info.buffer_size()];

        let tex_format = match reader.output_color_type() {
            (ColorType::Rgb, BitDepth::Eight) => TexturePixelFormat::RGB8,
            (ColorType::Rgb, BitDepth::Sixteen) => TexturePixelFormat::RGBA16UI,
            (ColorType::Rgba, BitDepth::Eight) => TexturePixelFormat::RGBA8,
            (ColorType::Rgba, BitDepth::Sixteen) => TexturePixelFormat::RGBA16UI,
            (ColorType::Grayscale, BitDepth::Eight) => TexturePixelFormat::R8,
            (ColorType::Grayscale, BitDepth::Sixteen) => TexturePixelFormat::R16UI,
            (_, _) => unimplemented!("Unsupported PNG Pixel Type"),
        };

        let new_tex = unsafe {
            gl.create_texture()
                .map_err(TextureError::CreateTextureFailed)?
        };

        let dimension = config.dimension.to_gl();

        unsafe {
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(dimension, Some(new_tex));

            let levels = {
                if config.generate_mipmap {
                    (info.width as f32).log2().ceil() as i32
                } else {
                    1
                }
            };

            gl.tex_parameter_i32(
                dimension,
                glow::TEXTURE_MAG_FILTER,
                config.mag_interpolation.to_gl(),
            );
            gl.tex_parameter_i32(
                dimension,
                glow::TEXTURE_MIN_FILTER,
                config.min_interpolation.to_gl(),
            );
            gl.tex_parameter_i32(
                dimension,
                glow::TEXTURE_WRAP_S,
                config.edge_behaviour.to_gl(),
            );

            gl.tex_parameter_i32(
                dimension,
                glow::TEXTURE_WRAP_T,
                config.edge_behaviour.to_gl(),
            );

            if dimension == glow::TEXTURE_3D {
                gl.tex_parameter_i32(
                    dimension,
                    glow::TEXTURE_WRAP_R,
                    config.edge_behaviour.to_gl(),
                );
            }

            if dimension == glow::TEXTURE_2D {
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
            }
            if let Dimension::D3(width, height, depth) = config.dimension {
                gl.tex_storage_3d(
                    glow::TEXTURE_3D,
                    levels,
                    tex_format.to_sized_internal_format(),
                    width as i32,
                    height as i32,
                    depth as i32,
                );

                gl.tex_sub_image_3d(
                    glow::TEXTURE_3D,
                    0,
                    0,
                    0,
                    0,
                    width as i32,
                    height as i32,
                    depth as i32,
                    tex_format.to_format(),
                    tex_format.to_type(),
                    glow::PixelUnpackData::Slice(image_pixels),
                );
            }

            if levels > 1 {
                gl.generate_mipmap(dimension);
            }
        }
        return Ok(Texture {
            tex: new_tex,
            format: tex_format,
            config: config,
        });
    }

    pub fn bind_to_uniform(
        &self,
        gl: &Context,
        texture_unit_id: u32,
        uniform: Option<&glow::UniformLocation>,
    ) {
        unsafe {
            gl.active_texture(texture_unit_id_to_gl(texture_unit_id));
            gl.bind_texture(self.config.dimension.to_gl(), Some(self.tex));
            // Tell WebGL which uniform refers to this texture unit
            gl.uniform_1_i32(uniform, texture_unit_id as i32);
        }
    }

    pub fn create_render_target(
        gl: &Context,
        config: TextureConfig,
        pixel_format: TexturePixelFormat,
    ) -> Result<Texture, TextureError> {
        // Creates a render target texture but does not set up storage. Call "resize_render_target" to set
        // up storage
        let new_tex = unsafe {
            gl.create_texture()
                .map_err(TextureError::CreateTextureFailed)?
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                config.mag_interpolation.to_gl(),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                config.min_interpolation.to_gl(),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                config.edge_behaviour.to_gl(),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                config.edge_behaviour.to_gl(),
            );
            assert_eq!(gl.get_error(), glow::NO_ERROR);
        }

        Ok(Self {
            tex: new_tex,
            format: pixel_format,
            config: config,
        })
    }

    pub fn resize_render_target(&self, gl: &Context, resolution: &[i32; 2]) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.tex));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                self.format.to_sized_internal_format() as i32,
                resolution[0],
                resolution[1],
                0,
                self.format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                self.format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                None, // but we are passing in None here, so the above two values are ignored.
            );

            if self.config.generate_mipmap {
                gl.generate_mipmap(glow::TEXTURE_2D);
            }

            assert_eq!(gl.get_error(), glow::NO_ERROR);
        }
    }
}

/// The precision and number of channels used for a buffer
/// Not all of these formats work in webGL. In my tests
/// RGBA8 and RGBA16F work on Chrome and Firefox.
/// For a list of supposedly working ones see:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum TexturePixelFormat {
    R8,
    R8_SNORM,
    R16F,
    R32F,
    R8UI,
    R8I,
    R16UI,
    R16I,
    R32UI,
    R32I,
    RG8,
    RG8_SNORM,
    RG16F,
    RG32F,
    RG8UI,
    RG8I,
    RG16UI,
    RG16I,
    RG32UI,
    RG32I,
    RGB8,
    SRGB8,
    RGB565,
    RGB8_SNORM,
    R11F_G11F_B10F,
    RGB9_E5,
    RGB16F,
    RGB32F,
    RGB8UI,
    RGB8I,
    RGB16UI,
    RGB16I,
    RGB32UI,
    RGB32I,
    RGBA8,
    SRGB8_ALPHA8,
    RGBA8_SNORM,
    RGB5_A1,
    RGBA4,
    RGB10_A2,
    RGBA16F,
    RGBA32F,
    RGBA8UI,
    RGBA8I,
    RGB10_A2UI,
    RGBA16UI,
    RGBA16I,
    RGBA32I,
    RGBA32UI,

    DEPTH_COMPONENT16,
}
impl TexturePixelFormat {
    pub fn to_sized_internal_format(&self) -> u32 {
        match self {
            Self::R8 => glow::R8,
            Self::R8_SNORM => glow::R8_SNORM,
            Self::R16F => glow::R16F,
            Self::R32F => glow::R32F,
            Self::R8UI => glow::R8UI,
            Self::R8I => glow::R8I,
            Self::R16UI => glow::R16UI,
            Self::R16I => glow::R16I,
            Self::R32UI => glow::R32UI,
            Self::R32I => glow::R32I,
            Self::RG8 => glow::RG8,
            Self::RG8_SNORM => glow::RG8_SNORM,
            Self::RG16F => glow::RG16F,
            Self::RG32F => glow::RG32F,
            Self::RG8UI => glow::RG8UI,
            Self::RG8I => glow::RG8I,
            Self::RG16UI => glow::RG16UI,
            Self::RG16I => glow::RG16I,
            Self::RG32UI => glow::RG32UI,
            Self::RG32I => glow::RG32I,
            Self::RGB8 => glow::RGB8,
            Self::SRGB8 => glow::SRGB8,
            Self::RGB565 => glow::RGB565,
            Self::RGB8_SNORM => glow::RGB8_SNORM,
            Self::R11F_G11F_B10F => glow::R11F_G11F_B10F,
            Self::RGB9_E5 => glow::RGB9_E5,
            Self::RGB16F => glow::RGB16F,
            Self::RGB32F => glow::RGB32F,
            Self::RGB8UI => glow::RGB8UI,
            Self::RGB8I => glow::RGB8I,
            Self::RGB16UI => glow::RGB16UI,
            Self::RGB16I => glow::RGB16I,
            Self::RGB32UI => glow::RGB32UI,
            Self::RGB32I => glow::RGB32I,
            Self::RGBA8 => glow::RGBA8,
            Self::SRGB8_ALPHA8 => glow::SRGB8_ALPHA8,
            Self::RGBA8_SNORM => glow::RGBA8_SNORM,
            Self::RGB5_A1 => glow::RGB5_A1,
            Self::RGBA4 => glow::RGBA4,
            Self::RGB10_A2 => glow::RGB10_A2,
            Self::RGBA16F => glow::RGBA16F,
            Self::RGBA32F => glow::RGBA32F,
            Self::RGBA8UI => glow::RGBA8UI,
            Self::RGBA8I => glow::RGBA8I,
            Self::RGB10_A2UI => glow::RGB10_A2UI,
            Self::RGBA16UI => glow::RGBA16UI,
            Self::RGBA16I => glow::RGBA16I,
            Self::RGBA32I => glow::RGBA32I,
            Self::RGBA32UI => glow::RGBA32UI,

            Self::DEPTH_COMPONENT16 => glow::DEPTH_COMPONENT16,
        }
    }

    pub fn to_format(&self) -> u32 {
        match self {
            Self::R8 => glow::RED,
            Self::R8_SNORM => glow::RED,
            Self::R16F => glow::RED,
            Self::R32F => glow::RED,
            Self::R8UI => glow::RED_INTEGER,
            Self::R8I => glow::RED_INTEGER,
            Self::R16UI => glow::RED_INTEGER,
            Self::R16I => glow::RED_INTEGER,
            Self::R32UI => glow::RED_INTEGER,
            Self::R32I => glow::RED_INTEGER,
            Self::RG8 => glow::RG,
            Self::RG8_SNORM => glow::RG,
            Self::RG16F => glow::RG,
            Self::RG32F => glow::RG,
            Self::RG8UI => glow::RG_INTEGER,
            Self::RG8I => glow::RG_INTEGER,
            Self::RG16UI => glow::RG_INTEGER,
            Self::RG16I => glow::RG_INTEGER,
            Self::RG32UI => glow::RG_INTEGER,
            Self::RG32I => glow::RG_INTEGER,
            Self::RGB8 => glow::RGB,
            Self::SRGB8 => glow::RGB,
            Self::RGB565 => glow::RGB,
            Self::RGB8_SNORM => glow::RGB,
            Self::R11F_G11F_B10F => glow::RGB,
            Self::RGB9_E5 => glow::RGB,
            Self::RGB16F => glow::RGB,
            Self::RGB32F => glow::RGB,
            Self::RGB8UI => glow::RGB_INTEGER,
            Self::RGB8I => glow::RGB_INTEGER,
            Self::RGB16UI => glow::RGB_INTEGER,
            Self::RGB16I => glow::RGB_INTEGER,
            Self::RGB32UI => glow::RGB_INTEGER,
            Self::RGB32I => glow::RGB_INTEGER,
            Self::RGBA8 => glow::RGBA,
            Self::SRGB8_ALPHA8 => glow::RGBA,
            Self::RGBA8_SNORM => glow::RGBA,
            Self::RGB5_A1 => glow::RGBA,
            Self::RGBA4 => glow::RGBA,
            Self::RGB10_A2 => glow::RGBA,
            Self::RGBA16F => glow::RGBA,
            Self::RGBA32F => glow::RGBA,
            Self::RGBA8UI => glow::RGBA_INTEGER,
            Self::RGBA8I => glow::RGBA_INTEGER,
            Self::RGB10_A2UI => glow::RGBA_INTEGER,
            Self::RGBA16UI => glow::RGBA_INTEGER,
            Self::RGBA16I => glow::RGBA_INTEGER,
            Self::RGBA32I => glow::RGBA_INTEGER,
            Self::RGBA32UI => glow::RGBA_INTEGER,

            Self::DEPTH_COMPONENT16 => glow::DEPTH_COMPONENT,
        }
    }

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
            Self::R32UI => glow::UNSIGNED_INT,
            Self::R32I => glow::INT,
            Self::RG8 => glow::UNSIGNED_BYTE,
            Self::RG8_SNORM => glow::BYTE,
            Self::RG16F => glow::FLOAT,
            Self::RG32F => glow::FLOAT,
            Self::RG8UI => glow::UNSIGNED_BYTE,
            Self::RG8I => glow::BYTE,
            Self::RG16UI => glow::UNSIGNED_SHORT,
            Self::RG16I => glow::SHORT,
            Self::RG32UI => glow::UNSIGNED_INT,
            Self::RG32I => glow::INT,
            Self::RGB8 => glow::UNSIGNED_BYTE,
            Self::SRGB8 => glow::UNSIGNED_BYTE,
            Self::RGB565 => glow::UNSIGNED_SHORT_5_6_5,
            Self::RGB8_SNORM => glow::BYTE,
            Self::R11F_G11F_B10F => glow::UNSIGNED_INT_10F_11F_11F_REV,
            Self::RGB9_E5 => glow::UNSIGNED_INT_5_9_9_9_REV,
            Self::RGB16F => glow::HALF_FLOAT,
            Self::RGB32F => glow::FLOAT,
            Self::RGB8UI => glow::UNSIGNED_BYTE,
            Self::RGB8I => glow::BYTE,
            Self::RGB16UI => glow::UNSIGNED_SHORT,
            Self::RGB16I => glow::SHORT,
            Self::RGB32UI => glow::UNSIGNED_INT,
            Self::RGB32I => glow::INT,
            Self::RGBA8 => glow::UNSIGNED_BYTE,
            Self::SRGB8_ALPHA8 => glow::UNSIGNED_BYTE,
            Self::RGBA8_SNORM => glow::BYTE,
            Self::RGB5_A1 => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA4 => glow::UNSIGNED_SHORT_4_4_4_4,
            Self::RGB10_A2 => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA16F => glow::HALF_FLOAT,
            Self::RGBA32F => glow::FLOAT,
            Self::RGBA8UI => glow::UNSIGNED_BYTE,
            Self::RGBA8I => glow::BYTE,
            Self::RGB10_A2UI => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA16UI => glow::UNSIGNED_SHORT,
            Self::RGBA16I => glow::SHORT,
            Self::RGBA32I => glow::INT,
            Self::RGBA32UI => glow::UNSIGNED_INT,

            Self::DEPTH_COMPONENT16 => glow::UNSIGNED_INT,
        }
    }

    #[allow(dead_code)]
    pub fn to_channel_count(&self) -> usize {
        match self {
            Self::R8 => 1,
            Self::R8_SNORM => 1,
            Self::R16F => 1,
            Self::R32F => 1,
            Self::R8UI => 1,
            Self::R8I => 1,
            Self::R16UI => 1,
            Self::R16I => 1,
            Self::R32UI => 1,
            Self::R32I => 1,
            Self::RG8 => 2,
            Self::RG8_SNORM => 2,
            Self::RG16F => 2,
            Self::RG32F => 2,
            Self::RG8UI => 2,
            Self::RG8I => 2,
            Self::RG16UI => 2,
            Self::RG16I => 2,
            Self::RG32UI => 2,
            Self::RG32I => 2,
            Self::RGB8 => 3,
            Self::SRGB8 => 3,
            Self::RGB565 => 3,
            Self::RGB8_SNORM => 3,
            Self::R11F_G11F_B10F => 3,
            Self::RGB9_E5 => 3,
            Self::RGB16F => 3,
            Self::RGB32F => 3,
            Self::RGB8UI => 3,
            Self::RGB8I => 3,
            Self::RGB16UI => 3,
            Self::RGB16I => 3,
            Self::RGB32UI => 3,
            Self::RGB32I => 3,
            Self::RGBA8 => 4,
            Self::SRGB8_ALPHA8 => 4,
            Self::RGBA8_SNORM => 4,
            Self::RGB5_A1 => 4,
            Self::RGBA4 => 4,
            Self::RGB10_A2 => 4,
            Self::RGBA16F => 4,
            Self::RGBA32F => 4,
            Self::RGBA8UI => 4,
            Self::RGBA8I => 4,
            Self::RGB10_A2UI => 4,
            Self::RGBA16UI => 4,
            Self::RGBA16I => 4,
            Self::RGBA32I => 4,
            Self::RGBA32UI => 4,

            Self::DEPTH_COMPONENT16 => 1,
        }
    }
}

fn texture_unit_id_to_gl(int: u32) -> u32 {
    assert!(int <= 32);
    glow::TEXTURE0 + int
}
