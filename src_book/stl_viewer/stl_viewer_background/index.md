# Background and Appearance

The background is black - pretty boring. And although the matcap looks OK,
it would be nice to have something a bit fancier. 


# Full Screen Quad
What is a background? Well, it's always behind everything in the scene, 
and it should have some sort of shading so that the user can see which 
way they are facing. Because it has to cover the entiere screen we can 
either use a box surrounding the scene (ie cubemap) or we can use a 
single full screen quad.

Fortunately we already know how to full screen quads as we've been 
doing them since the beginning. The only difference is that in the 
vertex shader we need to compute the world space coordinates of the 
vertices so that it can be textured, and we need to force the Z 
distance to be behind everything.

```glsl
#version 300 es
precision mediump float;

in vec2 vert_pos; // In Screen Space
out vec3 world_pos;

uniform mat4 world_to_camera;
uniform mat4 camera_to_screen;

void main() {
    
    mat4 camera_to_world = inverse(world_to_camera);
    mat4 screen_to_camera = inverse(camera_to_screen);
    mat4 screen_to_world = camera_to_world * screen_to_camera;
    
    vec4 world_pos_tmp = screen_to_world * vec4(vert_pos, 1.0, 1.0);
    world_pos = world_pos_tmp.xyz;

    gl_Position.xy = vert_pos.xy;
    gl_Position.z = 0.99999;
    gl_Position.w = 1.0;
}
```

# Texturing the full screen quad
Now we have the world position of each corner, and we need to find some
way to represent the world. Blender can render out equirectangular maps,
so we may as well use one of those. Just to be fancy, we can use encode
the brightness of the image in the alpha channel to allow encoding
brightness's higher than `1`. 

That all works out to:
```glsl
#version 300 es
precision mediump float;

in vec3 world_pos;
out vec4 FragColor;

uniform sampler2D image_matcap;


vec3 sample_background(vec3 pos, float mip) {
    vec2 coords = vec2(
        atan(pos.y, pos.x) / 3.14159,
        sin(pos.z)
    );
    coords = coords * 0.5 + 0.5;
    
    vec4 raw = textureLod(image_matcap, coords, mip);
    float luminance = pow(((1.0 / raw.a) - 1.0), 1.0 / 2.2);
    
    return raw.rgb * luminance;
}


void main() {
    vec3 matcap = sample_background(world_pos, 2.0);
    FragColor.rgb = matcap * 0.5;
    FragColor.a = 1.0;
}
```


# Cooler Model Lighting
It would be cool if we could use the same image to light the geometry.
How can we do this? Well, we need to approximate the diffuse lighting
and perhaps some specular as well. Diffuse lighting can be approximated
by sampling in the direction of the surface normal (perhaps from a smaller
mip to emulate a surface roughness). Specular highlights are caused by
reflections, so we can use the GLSL reflect function to find what direction
to sample in.

From there, we can fiddle with the strength and blending of these lighting
values with the surface color to achieve our result.


Using the same `sample_background` function as per before:
```glsl
void main() {
    vec3 diffuse = sample_background(world_nor, 3.0);
    
    vec3 reflect = reflect(vec3(0.0, 0.0, -1.0), screen_nor);
    vec4 reflect_world = (camera_to_world * vec4(reflect, 0.0));
    vec3 reflection = sample_background(reflect_world.xyz, 4.0);
    
    float fresnel = 1.0 - dot(screen_nor, vec3(0.0, 0.0, 1.0));
    
    vec3 out_col = color;
    out_col = out_col * diffuse;
    out_col += reflection * fresnel * 0.5;
    out_col *= 1.0 - fresnel * 0.5;
    
    FragColor.rgb = out_col;
    FragColor.a = 1.0;
}
```

And our final canvas is:

<canvas id="stl_viewer_background"></canvas>

Because I was developing this on my laptop away from a power socket,
I also changed the rendering to only render periodically, but to force
updates when the camera is moving:

```rust
    pub fn animation_frame(&mut self) {
        self.check_resize();
        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;
        
        
        let time_since_render = time - self.last_render_time;
        if time_since_render > 0.2 {
            self.dirty = true;
        }
        
        if self.dirty {
            self.render();
            self.dirty = false;
            self.last_render_time = time;
        }
    }
    
    ...
    
    // For example:
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
        self.dirty = true;
    }
    ...

```
