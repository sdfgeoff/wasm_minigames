# Ship Motion Physics

Currently the ship sprites are positioned by simple time-varying
functions. We need to switch this to being controlled by some sort of
physics. There are a couple parts to the physics:

1. Motion Dynamics (drag, inertia, application of thrust)
3. Collision Detection

This page will cover the motion dynamics.

---------------------------------

The first part is to define our in-game ship entity. The ship entity
needs to store the current position, the velocity, and the state of the engine.
To make it easier to render, the ship entity also contains it's color and to
allow the motion physics to be separated from the input/control logic, the
application of force/thrust is also a separate member:

```rust
pub struct Ship {
    pub position: Transform2d,
    pub velocity: Transform2d,
    pub linear_thrust: f32,
    pub turning_thrust: f32,
    pub color: (f32, f32, f32, f32),
}
```

Inside the game we can now create a vector of ships, and render it with
a single ship sprite:

```rust
pub struct App {
    ....
    ship_sprite: ShipSprite,
    ship_entities: Vec<Ship>,
    ....
}

<< snip >>

        // Render all the ships
        self.ship_sprite.world_to_camera = world_to_camera;
        self.ship_sprite.camera_to_clipspace = camera_to_clipspace;

        for ship in &self.ship_entities {
            self.ship_sprite.world_to_sprite = ship.position.to_mat3_array();
            self.ship_sprite.ship_color = ship.color;
            self.ship_sprite.ship_engine = ship.linear_thrust;
            self.ship_sprite.render(&self.gl);
        }
```

So now that we can see our ship entities, what does the motion physics
look like?

1. The engine should provide thrust in the direction the ship is facing
2. There should be damping/drag to slow the ship down

Conceptually:
```
acceleration -= k_drag * velocity
acceleration += ship_direction * thrust * k_thrust

velocity += acceleration * delta_time
position += velocity * delta_time
```

Turns out that's all that's really required:

```rust
{{#include ./src/ship.rs}}
```

Connect up some input to one of the ships:
```rust
    pub fn key_event(&mut self, event: KeyboardEvent) {

        let player_entity = &mut self.ship_entities[0];
        if event.code() == "KeyW" {
            player_entity.linear_thrust = 1.0;
        }
        if event.code() == "KeyS" {
            player_entity.linear_thrust = -1.0;
        }
        if event.code() == "KeyA" {
            player_entity.angular_thrust = 1.0;
        }
        if event.code() == "KeyD" {
            player_entity.angular_thrust = -1.0;
        }
    }
```

And we are good to go:

<canvas id="swoop_ship_motion_physics"></canvas>

You'll notice that once you start turning it keeps turning, that's because
we haven't yet turned the keypress events into something that cleanly
signals if the player is holding the key down or not.

I was also sneaky and defined the camera transform as the X/Y transform
of the player.
