# Passing In Uniforms

Uniforms are used to control a shader while it is running. They can pass in
data such as object transforms, the time, the screen resolution or anything
else really.

There are two parts to passing in a uniform:

1. Finding where the uniform is using `gl.get_uniform_location(&program, name)`
2. _When the program is active_ setting the value of the uniform using `gl.uniform*` to set the value.


I've also changed the triangle to being a single full-screen quad. This means
we can now do fancy pixel-shader-rendering:
<canvas id="passing_in_uniforms"></canvas>

Yes that's a single quad. The Shader is [taken from
shadertoy](https://www.shadertoy.com/view/tt2XzG), written [by
"iq"](https://www.iquilezles.org/) and used under CC-BY-NC-SA 3.0.


For this I passed in a floating point number for time, and a float vec2 for
resolution: 
```
fn get_uniform_location(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    name: &str,
) -> Result<WebGlUniformLocation, ShaderError> {
    gl.get_uniform_location(&program, name)
        .ok_or(ShaderError::MissingUniform(name.to_string()))
}

<< snip >>

let uniform_resolution = get_uniform_location(&gl, &program, "iResolution")?;
let uniform_time = get_uniform_location(&gl, &program, "iTime")?;

<< snip >>

gl.uniform1f(Some(&self.uniform_time), self.time);
gl.uniform2f(Some(&self.uniform_resolution), self.resolution.0 as f32, self.resolution.1 as f32);
```

There are some gotcha's. The uniform name has to exist in the shader and be used.
So if you have the a shader that declares `uniform float iTime` but then never
uses it, the uniform will be compiled out, and `get_uniform_location` will
return `None`.

Another gotcha is that the program must be active when you set the uniform value.
Otherwise you'll get a warning in console and nothing will happen.
