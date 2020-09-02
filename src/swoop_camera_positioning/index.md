# Camera Positioning

As part of the game, the user should try to avoid crashing into
walls. This should be humanly possible to do.
The ship travels at 4 units/s of velocity and from the center of the
screen to the top edge is 0.5 of a unit. This means that it takes
1/8th of a second for the map to change completely. Human response time 
is a bit slower - about 1/4 to 1/5 of a second. A fun racing game should
be on the limits of this as that is where a user is pushing the edge of
what they can do. To achieve this we need to have about one unit of 
distance between the edge of the screen and the players ship. There are 
two options:

1. Zoom out the camera. This could cause loss of visibility  as the players
ship becomes small
2. Place the ship off-center away from the ships center of motion. This
could be confusing when the player makes a sudden motion.

I think a combination of both will work best, using the players 
velocity to move the center position of the camera and to zoom out when 
the player is moving fast.

So let's create a rough outline of a system for camera positioning:
```rust
pub struct Camera {
    position: Vec2,
    zoom: f32,
    target_position: Vec2,
    target_velocity: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            zoom: 1.0,
            target_position: (0.0, 0.0),
            target_velocity: (0.0, 0.0),
        }
    }
    
    pub fn reset(&mut self) {
        self.position = (0.0, 0.0);
        self.zoom = 10.0; // Start zoomed out so there is a nice "zoom" animation at the game start
        self.target_position = (0.0, 0.0);
        self.target_velocity = (0.0, 0.0);
    }
    
    
    /// Set information about the entity the camera is tracking
    pub fn set_target_information(&mut self, pos: &Vec2, vel: &Vec2) {
        self.target_position.0 = pos.0;
        self.target_position.1 = pos.1;
        
        self.target_velocity.0 = vel.0;
        self.target_velocity.1 = vel.1;
    }
    
    ///
    pub fn get_camera_matrix(&self, base_resolution: f32) -> [f32; 9] {
        Transform2d::new(
            self.position.0,
            self.position.1,
            0.0,
            1.0 / base_resolution * self.zoom,
        ).to_mat3_array()
    }
    
    /// Update the position of the camera, moving it towards the target
    /// position.
    pub fn update(&mut self, dt: f32) {
        // Do something fancy in here to position the camera
        self.position.0 = self.target_position.0;
        self.position.1 = self.target_position.1;
    }
}
```

What do we put in the update function?

Well, the ideal position is slightly ahead of the player, so:
```rust
    let ideal_position = (
            self.target_position.0 + self.target_velocity.0 * PREDICT_FACTOR,
            self.target_position.1 + self.target_velocity.1 * PREDICT_FACTOR,
        );
```
And the ideal zoom level is to zoom out the faster the player moves:
```rust
let velocity = length(self.target_velocity);
let ideal_zoom = 1.0 + velocity * ZOOM_FACTOR;
```

To avoid the camera position moving wildly 
when the player changes the ships direction, the camera should move
smoothly towards the ideal position, so let's use a proportional 
controller:
```rust
let zoom_err = self.zoom - ideal_zoom;
let pos_err = (
    self.position.0 - ideal_position.0,
    self.position.1 - ideal_position.1,
);

self.zoom -= zoom_err * dt / SMOOTHING;

self.position.0 -= pos_err.0 * dt / SMOOTHING;
self.position.1 -= pos_err.1 * dt / SMOOTHING;
```

Now it's a case of fiddling constants to make it play nicely. You can
do some math to calculate constants to achieve exactly 1 unit of space
ahead of the player, but the end goal is for it to "feel nice" rather
than be precise. In the end, I found some nice constants were:
```rust
const PREDICT_FACTOR: f32 = 0.6;
const ZOOM_FACTOR: f32 = 0.125;
const SMOOTHING: f32 = 0.4;
```

The result is:

<canvas id="swoop_camera_positioning"></canvas>

Now compare it to the previous page. The game is exactly the same, but
you can probably fly around without crashing now!
