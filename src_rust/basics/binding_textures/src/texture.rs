use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{Blob, HtmlImageElement, Url, WebGlTexture, WebGlUniformLocation};

#[derive(Debug)]
pub enum TextureError {
    AllocateTextureError,
    JsError(JsValue),
}

impl From<JsValue> for TextureError {
    fn from(err: JsValue) -> TextureError {
        TextureError::JsError(err)
    }
}

/// Makes an HTMLImageElement display an image from a bunch of raw bytes.
/// This is useful if you have an image stored with `include_bytes!()`.
/// Assumes image is in PNG format
fn load_image_bytes_to_image_element(
    image_bytes: &[u8],
    img_element: &HtmlImageElement,
) -> Result<(), TextureError> {
    let raw_arr = unsafe { Uint8Array::view(image_bytes) };

    let arr = Array::new();
    arr.set(0, raw_arr.dyn_into().unwrap());

    let mut blob_options = web_sys::BlobPropertyBag::new();
    blob_options.type_("image/png");

    let blob: Blob = Blob::new_with_u8_array_sequence_and_options(&arr, &blob_options)?;

    let url = Url::create_object_url_with_blob(&blob)?;
    img_element.set_src(&url);

    Ok(())
}

pub fn load_texture(gl: &GL, image_bytes: &[u8]) -> Result<WebGlTexture, TextureError> {
    let texture = gl
        .create_texture()
        .ok_or(TextureError::AllocateTextureError)?;

    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    // Give our texture a default
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,                         // Level
        GL::RGBA as i32,           // Format
        1,                         // width
        1,                         // height
        0,                         // border
        GL::RGBA,                  // source format
        GL::UNSIGNED_BYTE,         // type
        Some(&[255, 0, 255, 255]), // pixels
    )?;

    let img_element = HtmlImageElement::new()?;

    let gl_clone = gl.clone();
    let img_element_clone = img_element.clone();
    let texture_clone = texture.clone();

    let onload = Closure::wrap(Box::new(move || {
        set_up_image(&gl_clone, &img_element_clone, &texture_clone);
    }) as Box<dyn Fn()>);

    img_element.set_onload(Some(onload.as_ref().unchecked_ref()));

    load_image_bytes_to_image_element(image_bytes, &img_element)?;

    onload.forget();

    Ok(texture)
}

/// Load an image from an HtmlImageElement to the GPU into the specified
/// texture object. Makes some assumptions about the type of image filtering...
pub fn set_up_image(gl: &GL, img_element: &HtmlImageElement, texture: &WebGlTexture) {
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

    gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        &img_element,
    )
    .expect("Loading Image Failed");
}

/// Binds a texture to a uniform and a specific texture unit. NOTE: This function
/// has several important things:
///  1) The shader program for the uniform must be active
///  2) The texture_unit parameter is a WebGl2RenderingContext::TEXTURE* constant
pub fn bind_2d_texture_to_uniform(
    gl: &GL,
    uniform: &Option<WebGlUniformLocation>,
    texture: &WebGlTexture,
    texture_unit: TextureUnit,
) {
    // Tell WebGL which texture unit we are configuring
    gl.active_texture(texture_unit.as_gl_const());
    // Tell WebGL what texture to load into the texture unit
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    // Tell WebGL which uniform refers to this texture unit
    gl.uniform1i(uniform.as_ref(), texture_unit.as_int());
}

/// Texture units are pieces of (possibly emulated) hardware on the GPU. For
/// a texture to be rendered by the GPU, the texture unit needs to be configured
/// to point at the image in memory.
/// WebGL ensures there are 4 texture units in the vertex shader and 8 in the
/// fragment shader. Some hardware may support more.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum TextureUnit {
    Unit0 = 0,
    Unit1 = 1,
    Unit2 = 2,
    Unit3 = 3,
    Unit4 = 4,
    Unit5 = 5,
    Unit6 = 6,
    Unit7 = 7,
    Unit8 = 8,
    Unit9 = 9,
    Unit10 = 10,
    Unit11 = 11,
    Unit12 = 12,
    Unit13 = 13,
    Unit14 = 14,
    Unit15 = 15,
    Unit16 = 16,
    Unit17 = 17,
    Unit18 = 18,
    Unit19 = 19,
    Unit20 = 20,
    Unit21 = 21,
    Unit22 = 22,
    Unit23 = 23,
    Unit24 = 24,
    Unit25 = 25,
    Unit26 = 26,
    Unit27 = 27,
    Unit28 = 28,
    Unit29 = 29,
    Unit30 = 30,
    Unit31 = 31,
}

impl TextureUnit {
    fn as_gl_const(&self) -> u32 {
        match self.as_int() {
            0 => GL::TEXTURE0,
            1 => GL::TEXTURE1,
            2 => GL::TEXTURE2,
            3 => GL::TEXTURE3,
            4 => GL::TEXTURE4,
            5 => GL::TEXTURE5,
            6 => GL::TEXTURE6,
            7 => GL::TEXTURE7,
            8 => GL::TEXTURE8,
            9 => GL::TEXTURE9,
            10 => GL::TEXTURE10,
            11 => GL::TEXTURE11,
            12 => GL::TEXTURE12,
            13 => GL::TEXTURE13,
            14 => GL::TEXTURE14,
            15 => GL::TEXTURE15,
            16 => GL::TEXTURE16,
            17 => GL::TEXTURE17,
            18 => GL::TEXTURE18,
            19 => GL::TEXTURE19,
            20 => GL::TEXTURE20,
            21 => GL::TEXTURE21,
            22 => GL::TEXTURE22,
            23 => GL::TEXTURE23,
            24 => GL::TEXTURE24,
            25 => GL::TEXTURE25,
            26 => GL::TEXTURE26,
            27 => GL::TEXTURE27,
            28 => GL::TEXTURE28,
            29 => GL::TEXTURE29,
            30 => GL::TEXTURE30,
            31 => GL::TEXTURE31,
            _ => panic!("Invalid texture unit"),
        }
    }

    fn as_int(&self) -> i32 {
        *self as i32
    }
}
