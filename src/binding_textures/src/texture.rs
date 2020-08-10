use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlImageElement, Url, WebGl2RenderingContext, WebGlTexture};

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

pub fn load_texture(
    gl: &WebGl2RenderingContext,
    image_bytes: &[u8],
) -> Result<WebGlTexture, TextureError> {
    let texture = gl
        .create_texture()
        .ok_or(TextureError::AllocateTextureError)?;

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    // Give our texture a default
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D,
        0,                                     // Level
        WebGl2RenderingContext::RGBA as i32,   // Format
        1,                                     // width
        1,                                     // height
        0,                                     // border
        WebGl2RenderingContext::RGBA,          // source format
        WebGl2RenderingContext::UNSIGNED_BYTE, // type
        Some(&[255, 0, 255, 255]),             // pixels
    )?;

    let raw_arr = unsafe { Uint8Array::view(image_bytes) };

    let arr = Array::new();
    arr.set(0, raw_arr.dyn_into().unwrap());

    let mut blob_options = web_sys::BlobPropertyBag::new();
    blob_options.type_("image/png");

    let blob: Blob = Blob::new_with_u8_array_sequence_and_options(&arr, &blob_options)?;

    let img_element = HtmlImageElement::new()?;
    let url = Url::create_object_url_with_blob(&blob)?;

    let gl_clone = gl.clone();
    let img_element_clone = img_element.clone();
    let texture_clone = texture.clone();

    let onload = Closure::wrap(Box::new(move || {
        gl_clone.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture_clone));

        gl_clone.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);

        gl_clone.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        gl_clone.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        gl_clone
            .tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &img_element_clone,
            )
            .expect("Loading Image Failed");
    }) as Box<dyn Fn()>);

    img_element.set_onload(Some(onload.as_ref().unchecked_ref()));
    img_element.set_src(&url);

    onload.forget();

    Ok(texture)
}
