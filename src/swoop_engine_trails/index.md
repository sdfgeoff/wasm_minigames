# Engine Trails

Wouldn't it be nice to know how far behind the guy ahead of you is?
That's why the shadertoy implementation has trails behind the ships.

This won't be done as a sprite, but it needs a whole strip of vertices
that will follow the path of the ship. We'll create a strip with fixed
positions and then use a uniform containing some description of the path
and a vertex shader to position the trail.



<canvas id="swoop_enemy_racers"></canvas>
