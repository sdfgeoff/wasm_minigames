# Collision Physics

## Collision with other ships

Physics is a slightly harder topic in Rust than it would be in other
languages because Rust cares about mutability. So when you go through
an array of objects and generate a set containing pairs of objects, Rust's
borrow checker starts yelling at you.
When you then try an iterate through the pairs and try have mutable
access to them, it yells even more.

Now, when writing the system, the programmer knows that he will never
try to mutate the same object at the same time, but how can we tell Rust
that?

I couldn't think of a good way, so I fell back on run-time reference 
and mutability checks - aka 
[`Rc`](https://doc.rust-lang.org/beta/std/rc/struct.Rc.html) and 
[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html).

`Rc` allows us to have multiple references to the same object, and `RefCell`
allows us to obtain mutability at runtime - but it will panic if something
else is already borrowing it. That's fine because that now makes it the
programmers job to ensure we don't try to call `borrow_mut()` twice.

So, we need to end up with a vector of pairs of `Rc<RefCell<&mut Ship>>>`
Why use `&mut Ship`? So that our ships stay safely in their existing
vector, and we just fiddle around with references to them. The easiest 
way to get this vector of pairs is to use the `itertools::permutations` 
which will generate all unique permutations of the vector.

The resulting code:
```rust
use itertools::Itertools;

let ship_refs = self.ship_entities.iter_mut().map(|x| Rc::new(RefCell::new(x)));
let all_pairs: ship_refs.permutations(2);
```

Now we can use `filter_map` to convert the "probable" collision pairs
into details of each collisions:
```rust
struct CollisionEvent<'a> {
    obj1: Rc<RefCell<&'a mut Ship>>,
    obj2: Rc<RefCell<&'a mut Ship>>,
    normal: (f32, f32),
    overlap: f32,
}

<< snip >>

let collisions = all_pairs.filter_map(|ships: Vec<Rc<RefCell<&mut Ship>>>| {
    let ship1 = ships[0].clone();
    let ship2 = ships[1].clone();

    let normal = vect_between(&ship1.borrow().position, &ship2.borrow().position);
    let len = length(normal);
    if len < SHIP_RADIUS {
        Some(CollisionEvent {
            obj1: ship1,
            obj2: ship2,
            normal: normalize(normal),
            overlap: len - SHIP_RADIUS,
        })
    } else {
        None
    }
});
```

And finally we can move the ships when they're colliding:
```rust
collisions.for_each(|pair| {
    let mut ship1 = pair.obj1.borrow_mut();
    let mut ship2 = pair.obj2.borrow_mut();
    
    ship1.position.x -= pair.normal.0 * pair.overlap * 0.5;
    ship1.position.y -= pair.normal.1 * pair.overlap * 0.5;
    ship2.position.x += pair.normal.0 * pair.overlap * 0.5;
    ship2.position.y += pair.normal.1 * pair.overlap * 0.5;
});
```

For convenience, some of these functions can be broken out of their
inline representation, but you have to specify lifetimes:
```
fn check_collision<'a>(ship1: Rc<RefCell<&'a mut Ship>>, ship2: Rc<RefCell<&'a mut Ship>>) -> Option<CollisionEvent<'a>>
```


## Collision with the map
When we rendered the map, we used a mathematical function to represent
the map. The advantage of this is that it means we can evaluate the map
on the CPU to determine collisions with the map.

In GLSL this function is:
```glsl
vec4 sin_consts_1 = vec4(0.2, 0.0, 0.0, 0.0);
vec4 sin_consts_2 = vec4(0.0, 0.0, 0.0, 0.0);
vec4 cos_consts_1 = vec4(0.0, -0.2, 0.0, 0.1);
vec4 cos_consts_2 = vec4(0.0, 0.0, 0.05, 0.0);


float map_function(vec2 position) {
    float course = length(position - vec2(0.0, 0.0));
    
    float angle = atan(position.x, position.y);
    vec4 angles_1 = vec4(angle, angle*2.0, angle*3.0, angle*4.0);
    vec4 angles_2 = vec4(angle*5.0, angle*6.0, angle*7.0, angle*8.0);
    
    float track_radius = track_base_radius;

    track_radius += dot(sin(angles_1), sin_consts_1);
    track_radius += dot(sin(angles_2), sin_consts_2);
    track_radius += dot(cos(angles_1), cos_consts_1);
    track_radius += dot(cos(angles_2), cos_consts_2);

    float track_sdf = course - track_radius;
    track_sdf = abs(track_sdf) - track_width;
    return track_sdf;
}
```

To make it easier to match (and pass values between), I converted this
to:
```glsl
const float sin_consts[8] = float[8](0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
const float cos_consts[8] = float[8](0.0, -0.2, 0.0, 0.1, 0.0, 0.0, 0.05, 0.0);


float map_function(vec2 position) {
    float course = length(position - vec2(0.0, 0.0));
    float angle = atan(position.x, position.y);
    float track_radius = track_base_radius;
    
    for (int i=0; i<8; i++) {
        float omega = float(i+1);
        track_radius += cos(angle * omega) * cos_consts[i];
        track_radius += sin(angle * omega) * sin_consts[i];
    }

    float track_sdf = course - track_radius;
    track_sdf = abs(track_sdf) - track_width;
    return track_sdf;
}
```

It's a bit less efficient (doesn't take advantage of the GPU being able
to do operations on multiple vector elements at the same time), but GPU
performance isn't likely to be an issue in this game, and it means that 
the rust code looks like:

```rust
    pub fn distance_field(&self, position: Vec2) -> f32 {
        let course = length(position);
        let angle = position.0.atan2(position.1);
        
        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }

        let mut track_sdf = course - track_radius;
        track_sdf = f32::abs(track_sdf) - self.track_width;
        return track_sdf;
    }
```

Easy to visually compare for correctness

Detecting if the ship is on/off the map is only half the problem. The
other part is getting the collision normal. One option would be to do
an analytical solution of the fourier series (which wouldn't be too
hard because it's a fourier series), or we can apply finite-difference.

```rust
    // Uses finite difference to approximate the normal. This isn't quite
    // the actual normal because the distance field isn't quite the distance
    // field.
    pub fn calc_normal(&self, position: Vec2) -> Vec2 {
        const DELTA: f32 = 0.01;
        let here = self.distance_field(position);
        let above = self.distance_field((position.0, position.1 + DELTA));
        let right = self.distance_field((position.0 + DELTA, position.1));
        
        let dx = right - here;
        let dy = above - here;
        
        return normalize((dx, dy));
    }
```
Yeah, I picked finite difference.

From here it's the very similar as for the ship collisions: move the
ship so that it's no longer colliding. I'll also add in a term to slow
the ship motion when colliding with a wall - just to encourage players
not to wall slide.
```rust
    // Collisions with map
    for ship in all_ships.iter_mut() {
        let map_sdf = map.distance_field((ship.position.x, ship.position.y));
        if map_sdf > -SHIP_RADIUS {
            let normal = map.calc_normal((ship.position.x, ship.position.y));
            let overlap = map_sdf + SHIP_RADIUS;
            
            // Place ship back on the map
            ship.position.x -= normal.0 * overlap;
            ship.position.y -= normal.1 * overlap;
            
            // Slow the ship down
            ship.velocity.x -= ship.velocity.x * dt * GROUND_FRICTION;
            ship.velocity.y -= ship.velocity.y * dt * GROUND_FRICTION;
        }
    }
```

And there we have it, simple physics completed:

<canvas id="swoop_ship_collision_physics"></canvas>

You'll notice you can get some jitter when pushing other ships into 
walls and corners. This is because we aren't doing any of the clever 
stuff normal physics engines do to allow object stacking. We'll see if 
that's a problem when we scale the map up.
