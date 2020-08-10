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
let uniform_resolution = gl.get_uniform_location(&program, "iResolution");
let uniform_time = gl.get_uniform_location(&program, "iTime");

<< snip >>

gl.use_program(Some(&self.program));

gl.uniform1f(self.uniform_time.as_ref(), self.time);
gl.uniform2f(
    self.uniform_resolution.as_ref(),
    self.resolution.0 as f32,
    self.resolution.1 as f32,
);
```

There are some gotcha's. The uniform name has to exist in the shader and be used.
So if you have the a shader that declares `uniform float iTime` but then never
uses it, the uniform will be compiled out, and `get_uniform_location` will
return `None`. Because the `gl.uniform*` functions can handle None, the result
is simply that it has no effect.

Another gotcha is that the program must be active (
`gl.use_program(Some(&self.program));`
) when you set the uniform value.
Otherwise you'll get a warning in console and nothing will happen.
