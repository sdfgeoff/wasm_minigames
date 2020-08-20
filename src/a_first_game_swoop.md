# A First Game: Swoop

Righto, we can draw things to the screen, we can grab input from the
user, all that remains is to create a game. I'm going to replicate
my shadertoy game "space racer":

<iframe width="640" height="360" frameborder="0" src="https://www.shadertoy.com/embed/WlScWd?gui=true&t=10&paused=true&muted=false" allowfullscreen></iframe>

In shadertoy there are no sprites, so everything there is drawn in a 
single full-screen squad (with some buffering for state). This limits 
what is possible and makes things like the AI and counting laps hard to 
do in a way that will run performantly. By using WebGL only for the 
rendering and using rust/wasm for the collisions/logic, we should be 
able to create a better game.
