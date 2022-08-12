# Enemy Racers

FLying by yourself around a map is pretty boring, it's time to make the
other players fly!

Lets just wire it in quickly with a function:
```rust
use super::ship::Ship;

pub fn calc_ai_control(ship: &mut Ship, skill: f32) {
    ship.linear_thrust = 1.0;
}
```
And putting it in the core gameloop:
```rust
for ship in self.ship_entities[1..].iter_mut() {
    calc_ai_control(ship, 1.0, &self.map);
}
```

Yes, this does mean there's no state in an AI player - but lets face it,
when you're driving around a map, you don't really consider what you just
did.
Another limitation is that we don't know where the other ships are. Due
to the simplicity of this game, I don't think this will be a problem.

We have a function on our map that we wrote when finding the start positions,
it's called `get_track_direction` and returns the direction the track is
facing for a given polar coordinate around the track. Similarly, using
the function `get_track_radius` which also takes in a polar angle, we
can figure out where the full polar coordinates of where the ship would
be if it were on the centerline of the track.

About here I noticed a bug that's been present for quite a while.
I thought the glsl function for atan was `atan(x, y)` and the rust atan 
function was `x.atan(y)`. But actually it's `atan(y, x)` and 
`y.atan(x)`. Because I've used it consistently wrong, pretty much the 
only effect is that my track polar coordinates are out by 90 degrees. 
Whoops.

I haven't gone through and fixed the previous pages because 
functionally there's no difference, but here's the diff:
```diff
diff src/swoop_camera_positioning/src/map.rs src/swoop_enemy_racers/src/map.rs
34c34
<         let angle = position.0.atan2(position.1);
---
>         let angle = position.1.atan2(position.0);
59c59
<         const ANGLE: f32 = 0.0;
---
>         const ANGLE: f32 = std::f32::consts::PI / 2.0;
72c72
<         let delta_radius = radius_here - radius_a_bit_further;
---
>         let delta_radius = radius_a_bit_further - radius_here;
84c84
<             -angle - extra_angle
---
>             angle - extra_angle - std::f32::consts::PI / 2.0
86c86
<             -angle + extra_angle + std::f32::consts::PI
---
>             angle + extra_angle + std::f32::consts::PI / 2.0


diff src/swoop_camera_positioning/src/resources/map.frag src/swoop_enemy_racers/src/resources/map.frag
24c24
<     float angle = atan(position.x, position.y);
---
>     float angle = atan(position.y, position.x);

```

Right, that out of the way, we can animate our AI's flying around the 
map by writing directly to their position/rotations:
```rust
    let polar_position = PolarCoordinate::from_cartesian((ship.position.x, ship.position.y));

    let track_angle_here = map.get_track_direction(polar_position.angle);
    let track_radius_here = map.track_radius(polar_position.angle);

    let track_polar_here = PolarCoordinate {
        angle: polar_position.angle,
        radius: track_radius_here
    };
    let track_centerline_here = track_polar_here.to_cartesian();

    ship.position.x = track_centerline_here.0;
    ship.position.y = track_centerline_here.1;
    ship.position.rot = track_angle_here;
    ship.linear_thrust = 0.1;
```

However, directly writing the position to the centerline of the track
is a bit of a cheat. We should control the AI with the same controls the
player has: the `linear_thrust` and `angular_thrust` variables.

So we have to find a way to convert "ideal" position into a set of 
control inputs. We can divide this problem into two "rules" for the AI:

1. Face in the direction of the track
2. Try to stay in the center of the track

We can compute the difference from facing along the track, and apply
it as a steering input:

```rust
    let angular_error = wrap_angle(track_angle_here - ship.position.rot);
    ship.angular_thrust = f32::max(f32::min(angular_error, 1.0), -1.0);
```

This fulfills rule 1, but the AI tends to wallslide. So lets compute 
a radius error and apply that:
```rust 
    let polar_position = PolarCoordinate::from_cartesian((ship.position.x, ship.position.y));

    let track_angle_here = map.get_track_direction(polar_position.angle);
    let track_radius_here = map.track_radius(polar_position.angle);
    
    let mut steering = 0.0;
    let mut thrust = 0.0;
    
    let radius_error = track_radius_here - polar_position.radius;
    let radius_steering_input = f32::max(f32::min(radius_error, PI / 2.0), -PI / 2.0);
    
    let mut target_angle = 0.0;
    target_angle += track_angle_here; // Face direction of track
    target_angle += radius_steering_input;  // Fly towards track center
    
    let angular_error = wrap_angle(target_angle - ship.position.rot);
    steering += angular_error;
   
    thrust += 1.0;

    ship.angular_thrust = f32::max(f32::min(steering, 1.0), -1.0);
    ship.linear_thrust = f32::max(f32::min(thrust, 1.0), -1.0);
```
Note that the addition of the radius_error is done by offsetting the 
angle we want the ship to fly in rather than direclty influencing the 
steering input. (It's worth noting that the angular error is a kind-of 
feed-forward for the radius error)

The ships now fly around, but still hit the walls lots. This is
because the ships can't see ahead of them. They are flying looking at
how far they are from the walls NOW rather than looking ahead and giving
the ship time to turn. We can fix this by instead of using the ship's current
position to drive the control system, we can use a simple prediction of
the ships position to drive it:

```rust
    let future_position = (
        ship.position.x + ship.velocity.x * LOOKAHEAD_TIME,
        ship.position.y + ship.velocity.y * LOOKAHEAD_TIME,
    );
    
    let polar_position = PolarCoordinate::from_cartesian(future_position);

    let track_angle_here = map.get_track_direction(polar_position.angle);
    let track_radius_here = map.track_radius(polar_position.angle);
    
    let mut steering = 0.0;
    let mut thrust = 0.0;
    
    let radius_error = track_radius_here - polar_position.radius;
    let radius_steering_input = f32::max(f32::min(radius_error, PI / 2.0), -PI / 2.0);
    
    let mut target_angle = 0.0;
    target_angle += track_angle_here; // Face direction of track
    target_angle += radius_steering_input;  // Fly towards track center
    
    let angular_error = wrap_angle(target_angle - ship.position.rot);
    steering += angular_error;
   
    thrust += 1.0;

    ship.angular_thrust = f32::max(f32::min(steering, 1.0), -1.0);
    ship.linear_thrust = f32::max(f32::min(thrust, 1.0), -1.0);
```

With a lookahead time of 0.5 seconds, the ship navigates the map on a
pretty nice racing line - turning in close on corners etc. However, 
now the ship can only see where it is 0.5 seconds ahead, and doesn't
know where it is now, so when the course is really twisty/turny, it can
still hit the wall.
So let's compute the steering input for both now and for the future:

```rust
pub fn calc_ai_control(ship: &mut Ship, _skill: f32, map: &Map) {
    let mut steering = 0.0;
    let mut thrust = 0.0;
    
    steering += calc_steering_input(&ship, &map, 1.0) * 0.15;
    steering += calc_steering_input(&ship, &map, 0.5) * 0.45;
    steering += calc_steering_input(&ship, &map, 0.2) * 0.4;
   
    thrust += 1.0;

    ship.angular_thrust = f32::max(f32::min(steering, 1.0), -1.0);
    ship.linear_thrust = f32::max(f32::min(thrust, 1.0), -1.0);
}


fn calc_steering_input(ship: &Ship, map: &Map, lookahead_time: f32) -> f32 {
    let polar_position = PolarCoordinate::from_cartesian(
        predict_position(ship, lookahead_time)
    );

    let track_angle_here = map.get_track_direction(polar_position.angle);
    let track_radius_here = map.track_radius(polar_position.angle);
    
    let mut steering = 0.0;
    
    let radius_error = track_radius_here - polar_position.radius;
    let radius_steering_input = f32::max(f32::min(radius_error, PI / 2.0), -PI / 2.0);
    
    let mut target_angle = 0.0;
    target_angle += track_angle_here; // Face direction of track
    target_angle += radius_steering_input;  // Fly towards track center
    
    let angular_error = wrap_angle(target_angle - ship.position.rot);
    steering += angular_error;
   
    steering
}

fn predict_position(ship: &Ship, time: f32) -> Vec2 {
    (
        ship.position.x + ship.velocity.x * time,
        ship.position.y + ship.velocity.y * time,
    )
}
```

We now have an AI that can fly inhumanly well. It nails every corner,
and as a result it's no fun to race against (and all the AI's are the 
same). Time to add some imperfections. Let's use the skill parameter to
vary the ships and reduce their lookahead.

```rust
let num_ships = self.ship_entities.len() - 2;
for (id, ship) in self.ship_entities[1..].iter_mut().enumerate() {
    let skill = id as f32 / num_ships as f32;
    calc_ai_control(ship, skill, &self.map);
}

<< snip >>

pub fn calc_ai_control(ship: &mut Ship, skill: f32, map: &Map) {
    let mut steering = 0.0;
    let mut thrust = 0.0;
    
    let lookahead_mul = skill;
    
    steering += calc_steering_input(&ship, &map, 1.0 * lookahead_mul) * 0.15;
    steering += calc_steering_input(&ship, &map, 0.5 * lookahead_mul) * 0.45;
    steering += calc_steering_input(&ship, &map, 0.2 * lookahead_mul) * 0.4;
   
    thrust += 1.0;

    ship.angular_thrust = f32::max(f32::min(steering, 1.0), -1.0);
    ship.linear_thrust = f32::max(f32::min(thrust, 1.0), -1.0);
}
```

And there we have it:

<canvas id="swoop/swoop_enemy_racers"></canvas>

I can always beat yellow (no lookahead), often beat yellow (0.5s max 
lookahead) and am not ever close to purple (1s lookahead). I suppose 
this makes sense because the players camera only gives about 0.3-0.5 
seconds of lookahead, so while the AI flies using the same ship 
limitations as a human, it can see more of the map...
