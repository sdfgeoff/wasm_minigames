# Binding Textures

Being able to run a shader is one thing, but a lot of the time we also need
to load in image textures. The way Mozilla suggests in their tutorials is to
use a XMLHttpRequest to fetch the image. However, because we're in a compiled
language we should be able to compile the image into the WASM blob. This isn't
necessarily always a good solution, but for small games it ensures that the
texture will be available at the same time as the WASM is loaded.

[`gl.texImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
is the function that is used to actually bind the texture. It can take input
in a whole bunch of forms, but most of these forms require uncompressed image data
or an HTML element. Rather than decompress the image in Rust, or decompress it
before including in the binary, we can get the browser to do it for us - we just
need some way to tell the browser to use data from our WASM blob as an image.

Turns out there's some hoops to jump through to get a browser to load an image
from binary data:

1. Convert the bytes into a Javascript Uint8Array
2. Create a "Blob" object from our Uint8Array
3. Create a URL so that html can "find" the image
4. Point an HTMLImageElement at the url pointing at the blob.

That translates to:

```rust
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
```

But loading the HTMLImageELement is asynchronus, so outside all of that we need
to:

1. Create a blank texture on the GPU
2. Start loading the image element
3. Substitute in the image when it's done.


```rust
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
```

You may notice I broke out #3 (substitude in the image when it's done) to a new
function called "set up image". This is because WebGL needs to know even more
about the image!!!! It needs to know how the shader should sample it, if MipMaps
should be generated.....

```rust
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
```
It's worth mentioning that although we use
`load_image_bytes_to_image_elements` here, it is trivial to remove that and
instead use the `set_src` of the `image_element` to load a URL instead. This
would be useful if you have lots of textures and need to stream them in
dynamically, but for the sorts of games I plan to make it isn't really needed.


Oookay, we should be ready to go now, right? Well....
Lets create a shader that uses a some textures:

```glsl
{{#include ../../../src_rust/basics/binding_textures/src/resources/shader.frag}}
```
The `vec2 uv = screen_pos.xy * 0.5 - 0.5` is because the `screen_pos` variable goes
from -1 to 1, but texture coordinates in the `texture` function go from 0 to 1.
I'm using two textures just so I can check the binding is happening correctly
(if there is any texture in any texture unit, an unbound sampler seems to use it?!)

Now we need to tell our shader program to use our texture
```rust
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
```

What's that TextureUnit thing? It's just a handy enum mapping
`GL::TextureUnit1` to the integer `1` and making it type safe....


And in our render function we can finally pass in a texture to the shader:
```rust
pub fn render(&mut self, gl: &WebGl2RenderingContext) {
    gl.use_program(Some(&self.program));

    gl.uniform1f(self.uniform_time.as_ref(), self.time);
    gl.uniform2f(
        self.uniform_resolution.as_ref(),
        self.resolution.0 as f32,
        self.resolution.1 as f32,
    );

    bind_2d_texture_to_uniform(
        &gl,
        &self.uniform_image_texture_1,
        &self.image_texture_1,
        TextureUnit::Unit0,
    );
    bind_2d_texture_to_uniform(
        &gl,
        &self.uniform_image_texture_2,
        &self.image_texture_2,
        TextureUnit::Unit1,
    );

    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&self.position_buffer),
    );

    gl.vertex_attrib_pointer_with_i32(
        self.attrib_vertex_positions,
        2, // num components
        WebGl2RenderingContext::FLOAT,
        false, // normalize
        0,     // stride
        0,     // offset
    );
    gl.enable_vertex_attrib_array(self.attrib_vertex_positions);

    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLE_STRIP,
        0, //offset,
        4, // vertex count
    );
}
```

After all that confuffling, The end result is:

<canvas id="basics/binding_textures"></canvas>
