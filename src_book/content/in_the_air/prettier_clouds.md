# Prettier Clouds
Before we worry about performance, lets make our clouds look a bit more passable.
For this I can grab my shadertoy code.

<canvas id="in_the_air/prettier_clouds"></canvas>

One of the features I used in Shadertoy is a 3D noise texture. So I had to add support for this. Mostly this was changing TEXTURE_2D's for TEXTURE_3D's, except for the actual loading texture data. Fortunately I don't actually care about loading valid texture data because I am loading white noise, so I just kinda ignored the whole problem. Please don't use my code as an example of how to do texture3D handling!

From then there was a bunch of twiddling to try get things to look good. My
shader code has lots of parameters, but is very poorly parameterized (lots of coupling). One day I'd like to rewrite it, but not today. For now it is good enough.