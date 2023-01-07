# A second game: Pilot
Way back in the dawn of time™ I created a game called Sky Pilot where you
land some sort of VTOL craft on a platform in the middle of the sky:

<iframe width="560" height="315" src="https://www.youtube.com/embed/qkFDz9JTE9Y" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

When I made that, my computer didn't support GLSL, so no dynamic shadows!

Anyway, that game is a bit simplistic, but I like the idea of combining
it with some recent cave-demos I did on shadertoy:

<iframe width="640" height="360" frameborder="0" src="https://www.shadertoy.com/embed/WdGBWW?gui=true&t=10&paused=true&muted=false" allowfullscreen></iframe>

So, the game will be:
1. Pilot a spacecraft
2. Avoid bumping into things too hard
3. Find and land on a platform
4. Don't run out of fuel

There are many complexities in this game:
1. Have to render/load meshes
2. Have to render signed distance fields and combine them with meshes
3. Have to do physics with the world signed distance field
4. Have to do physics with the platform mesh

And I'm going to try do it for the ProcJam 2020 and 7DFPS of which 
there are ... 7 days left.
I'm not that fussed if I don't finish in time.

# The Game Plan
Unlike the initial demos and the swoop game which were small enough
that we could launc into them without a clear idea of where we were going,
this game requires a bit more planning - both on the gameplay/content
side and on the Rust architecture side.


# Abandoned?
Yeah, I never managed to get the signed distance field of the cave to
render properly alongside the geometry of the player ship and other
assets. It's take some sitting down and figuring out how projection
matrices actually work :)

So anyway, I'll stick this up here because it contains my initial attempts
at deferred rendering - which may well be useful later (in fact,
I've referred to this example countless times when experimenting with other
openGL programs such as <a href="http://sdfgeoff.space/pages/scalable_editable_raster_graphics__serg_/index.html">sverg</a> and <a href="http://sdfgeoff.space/pages/gametoy__a_shadertoy_alike_for_making_games/index.html">gametoy</a>)

# Somewhat Unabandoned
Uhm, 2 years on I managed to get the signed distance field to render properly!
Yay! But I don't think I'll pick up the game as was originally planned here.
I have other things to build....