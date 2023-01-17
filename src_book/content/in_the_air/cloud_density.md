# In The Air Cloud Density

Currently our clouds are ... a bunch of evenly spaced spheres. That isn't very
cloud-like. Let's have a think about how we can make this better.

Let's focus on the overall shape of clouds first. They can be divided up by altitude. This great wikimedia image illustrates it nicely. 

![Valentin de Bruyn / CotonThis illustration has been created for Coton, the cloud identification guide for mobile., CC BY-SA 3.0 <https://creativecommons.org/licenses/by-sa/3.0>, via Wikimedia Commons](Cloud_types_en.svg)

As far as I can see the clouds are either thin and wispy (eg stratus) or chunky (eg cumulonimbus). Generally higher clouds are wispier.

From my own observation, clouds tend to come in layers. So why don't we map the
various image channels to different altitude levels. We can then use the value
to indicate the shape of the cloud:

 - 0 (no cloud)
 - 0.4999(no cloud)
 - 0.501 (very wispy cloud at the altitude floor)
 - 0.999 (very chunky cloud reaching from the bottom of one layer to the start of the next)

Why am I wasting all that space from 0 zero till 0.499? I have a suspiscion that I 
can use it like a signed distance field to speed up the raymarching later.

So here's how it will look:

![How cloud layers will be constructed from a single image](cloud_layer_plan.svg)

Each channel has 256 possible values, This is mapped with 1 unit = 10m so each
channel covers a vertical space of 2.56km. However each channel has a base 
spacing of 1.5km so there is an overlap of 1.56km to allow for more interesting
cloudscapes.

To reach a 6km rendering distance with 100 raymarcher steps, each step has to be 
60m. If high altitude clouds are never rendered, we may reduce the vertical scale
to increase possible cloudscape complexity. I'll have a twiddle with the cloud layer stretch and spacing once I have a prototype.

So let's do this! Create a cloud texture, stretch it so that the pixels are 60m
of worldspace (so a 128px square covers 7.6km), and turn it into a density map.
Why am I going with such a small texture? Well, this function is going to get
sampled a lot, so if it fits into the GPU's cache, then it will run WAY faster.

<canvas id="in_the_air/cloud_density"></canvas>

