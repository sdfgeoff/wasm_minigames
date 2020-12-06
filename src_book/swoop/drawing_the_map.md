# Drawing The Map

The map is the background for the everything, so we have two options:

1. Use a plane the size of the map and use the same transform stack as 
for the player ship.
2. Use a plane the size of the screen and shift the UV coordinates to
match the transform stack.

If we were doing an infinite map (ie some sort of exploration game) we 
would have to go with #2, but because we are doing a racing game where
the map is well bounded, solution #1 works just fine and saves a bunch
of effort.

So let's just copy our player sprite code and make it draw a bigger plane
with a different shader. We'll shunt the functions that handle uploading
the shader into a module `shader.rs`, but we because this is a small
game I won't bother trying to generalize the sprite code. Pretty much
the only code in the `ship_sprites.rs` and `map_sprite.rs` is to do
with handling uniforms - which is likely to be pretty shader specific.

```
{{#include ../../src_rust/swoop/swoop_drawing_the_map/src/shader.rs}}
```


So anyway, here's drawing the coordinates for the map:

<canvas id="swoop_drawing_the_map-0" options="coords"></canvas>


------------------------------------


You may think we would use a texture for the map, just as we did for
the player ship, however the map has slightly different requirements.
As well as being used to show the player where to go, we need to be
able to query the map and find out if a certain area is on the track
or not. While sampling an image is possible, it will be easier to
define the map with a mathematical function. This function can then
be evaluated on the CPU or GPU and will give the same results.

So what function should we use to draw the map? If the map function
returns an approximate distance to the racetrack, then we can use 
finite difference (or possibly an analytic solution) to resolve 
collision normals. So we want a function of the form:
```glsl
float map_distance_field = map_function(vec2 position)
```

The racetrack should loop back on itself, so it's basic form should
be a circle. We can then distort the circle to make the course more
interesting to race around using a fourier series.

So how do we get the signed distance field for a circle? Well, the
distance from a single point is a good start:

```glsl
float course = length(position - vec2(0.0, 0.0));
```
We're going to define our distance field as negative values being a drivable
area and positive values being walls. (aka distance to the track).
So lets expand our circle by the track radius:
```glsl
float track_sdf = course - track_radius;
```

To make things clearer while debugging, let's threshold it so we can
see where the track edges are:
```glsl
FragColor = vec4(vec3(track_sdf > 0.0), 1.0);
```

This gives us:

<canvas id="swoop_drawing_the_map-1" options="circle_1"></canvas>

You can see there's a black circle in the middle of the screen. This
would mean that the player can fly anywhere in that circle. We want the
player in a track, not an arena.

To turn it into a ring, we can use the abs function to make it 
symmetric around the current edge, and then offset it to reintroduce 
some negative (track) area:
```glsl
track_sdf = abs(track_sdf) - track_width;
```

<canvas id="swoop_drawing_the_map-2" options="circle_2"></canvas>

(Note that the blue ship is invisible because the ships use additive
blending)

Don't understand what is happening here? You're probably not alone.
Signed distance fields (SDF's) are a bit counter-intuitive at first.
I can't think of a good way to explain it, but it should become
evident how it works fairly quickly if you open up shadertoy and have 
a play yourself.

Flying around a circular track isn't very interesting, so we can use 
a fourier series to distort it based on the angle from the center:

```
{{#include ../../src_rust/swoop/swoop_drawing_the_map/src/resources/map_fourier_1.frag}}
```

And the resulting track:

<canvas id="swoop_drawing_the_map-3" options="fourier_1"></canvas>

It shouldn't be hard to port the map function into rust when it comes
time to write the collision detection.

Now to make it look pretty by adding a grid in the background and
drawing some lines around the edge:

<canvas id="swoop_drawing_the_map-4" options="visualized"></canvas>

Looks like a pretty small map? That's OK, we can tweak it using the
`track_width` and `track_base_radius` parameters later.

The final map rendering shader is:
```
{{#include ../../src_rust/swoop/swoop_drawing_the_map/src/resources/map_visualized.frag}}
```
