# STL Viewer Camera Lookaround

The user should be able to use the mouse to rotate around the object.
For this we need a more than just a single matrix to use for model
transformation. So we need to introduce mat4's to our rust. Rather than 
reinvent the wheel like I did for `swoop`, I'll use glam as it seems to 
do what I want.

In fact we need three matrices: The camera transform, the object transform,
and a camera-space-to-clip-space transform. 

The struct containing the STL mesh can contain the world_to_model 
matrix, but the camera matrices should be stored elsewhere.

# The Camera Matrix
We want the camera to rotate around the center of the scene, so it makes
sense to store the camera position as an elevation and azimuth and only
do the conversion when we need the matrix.

So if we store the camera as:

```rust
pub struct Camera {
    pub elevation: f32,
    pub azimuth: f32,
    pub distance: f32,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}
```

Then we can use the functions that glam provides us with to generate
the generate both the camera matrix and to generate the matrix from
the cameras position:
```rust
    /// Converts to world_to_camera and camera_to_screen matrices
    pub fn to_matrices(&self) -> (Mat4, Mat4) {
        let sa = f32::sin(self.azimuth);
        let ca = f32::cos(self.azimuth);
        let se = f32::sin(self.elevation);
        let ce = f32::cos(self.elevation);
        let position = Vec3::new(
            self.distance * ca * ce,
            self.distance * sa * ce,
            self.distance * se
        );
        let world_to_camera = Mat4::look_at_rh(
            position,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        
        let camera_to_screen = Mat4::perspective_rh_gl(
            self.fov,
            self.aspect,
            self.near,
            self.far,
        );
        
        (world_to_camera, camera_to_screen)
    }
```

## Mouse Look
We now need to set the elevation and azimuth based on the mouse clicking
and dragging. Currently we have all the mouse events going to the same 
callback, but now we need to distinguish mouse down from mouse up and
mouse move. This was just a case of repeating the stuff found in the
binding events page.

Now we can implement the logic that computes how much the user has
moved the mouse each frame the mouse is held down. We do this by storing
the location of the mouse on the previous frame:
```rust
struct App {
    ...
    click_location: Option<(i32, i32)>,
    ...
}
```

When the user is not pressing the mouse the value is `None`, and when 
the user is pressing the mouse, the value is the screen coordinates of 
the mouse position on the previous frame. Now inside the `mouse_move` 
callback we can compute the change in position and apply that to the 
cameras orientation.

```rust
pub fn mouse_move(&mut self, event: MouseEvent) {
        const DRAG_SENSITIVITY: f32 = 5.0;
        match self.click_location {
            Some(location) => {
                
                let new = (event.client_x(), event.client_y());
                let delta = (location.0 - new.0, location.1 - new.1);
                self.click_location = Some(new);
                
                let percentage_x = (delta.0 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;
                let percentage_y = (delta.1 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;
                
                self.camera.azimuth += percentage_x;
                self.camera.elevation -= percentage_y;
                self.camera.elevation = f32::min(f32::max(self.camera.elevation, -1.4), 1.4);
                
            }
            None => {
            }
        }
    }
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
    }
    pub fn mouse_up(&mut self, _event: MouseEvent) {
        self.click_location = None;
    }
```

## Updating the Vertex Shader
We have all three matrices, and can pass them into the vertex shader,
but what then?

```glsl
#version 300 es

precision mediump float;
in vec3 vert_pos;
in vec3 vert_nor;

out vec3 screen_pos;
out vec3 screen_nor;

uniform mat4 world_to_camera;
uniform mat4 world_to_model;
uniform mat4 camera_to_screen;

void main() {
    mat4 model_to_world = inverse(world_to_model);
    mat4 model_to_camera = world_to_camera * model_to_world;
    mat4 model_to_screen = camera_to_screen * model_to_camera;
    
    vec4 pos = vec4(vert_pos, 1.0);
    vec4 nor = vec4(vert_nor, 0.0);

    pos = model_to_screen * pos;
    nor = model_to_camera * nor;        
    
    screen_pos = pos.xyz / pos.w;
    screen_nor = nor.xyz;

    gl_Position.xyz = screen_pos;
    gl_Position.w = 1.0;
}
```


We can test this transforms by rendering the same mesh at two different
locations and checking that the camera moves around as we would expect:
```rust
        let (world_to_camera, camera_to_screen) = self.camera.to_matrices();
        self.shader_stl.setup(&self.gl, world_to_camera, camera_to_screen);
        
        self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, -25.0, 0.0));
        self.stl.render(&self.gl, &self.shader_stl);
        self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, 25.0, 0.0));
        self.stl.render(&self.gl, &self.shader_stl);
```

And the result is:

<canvas id="stl_viewer_camera_lookaround"></canvas>

