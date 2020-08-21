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
{{#include ./src/shader.rs}}
```

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

```
float course = length(position - vec2(0.0, 0.0));
```
This gives us:


<canvas id="swoop_drawing_the_map"></canvas>

