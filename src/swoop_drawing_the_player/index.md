# Drawing The Player Ship

In the [shadertoy game](https://www.shadertoy.com/view/WlScWd), the 
ship is drawn using a signed distance field. That's because you can't 
load custom textures. Here, [we just learned how to bind 
textures](../binding_textures/index.md).
We still want our textures to contain a gradient (as that is what was used
for the neon effect), but now we can draw the ship in a graphics program
like Gimp rather than hand-coding lines as a bunch of coordinates.

We have several textures we need:

1) The player ship
2) The player ship's engine
3) The start-box and start line where all the ships start
4) The "traffic light" that signals when the race starts

Instead of putting #1 and #2 in different textures we can have a single
player texture that contains the ship in one channel and the engine in
the other.

Here's the player ship texture:

![Player Ship Texture](./src/resources/ship.png)

So the ship is in the red channel and the engine in the blue channel.
I've put a circle in the green channel which could maybe be used to
indicate the collision box when the player hits the wall.

Now we need to render it.


<canvas id="swoop_drawing_the_player"></canvas>
