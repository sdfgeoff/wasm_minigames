# A First Shader

Now that we can get input into our game, it's time to display output
for the user. We'll be using 
[WebGL2](https://www.khronos.org/registry/webgl/specs/latest/2.0/).

Mozilla provides a great bunch of tutorials on webgl, the first of which is
[here](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL)
This (and the next few pages) are heavily based on these tutorials.

A HTML canvas con be a whole bunch of things, only one of which is
webgl. As a result, we have to specifically fetch webgl2 from the canvas:
```rust
fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
```

That's the easy part. From their to the first triangle is quite a long
way. The reason it is so complex is because it is a complex thing. We
need to:
1. Provide a matching vertex and fragment shader (that compile with no errors)
2. Provide a bunch of vertices for the shader to operate on

Porting from the Mozilla tutorials wasn't too hard, but:
1. Because Rust is amazing, you have to in a bunch of error checking for JS errors
2. I stripped out all the uniforms for now to make this example simpler
3. Because Rust doesn't seem to have the mat4 object, I removed the perspective matrix projection from the vertex shader

After that, we have:

<canvas id="a_first_shader"></canvas>

A triangle!

Most of the ported code for is in the file `triangle.rs`:
```rust
{{#include ../src/games/a_first_shader/src/triangle.rs}}
```

