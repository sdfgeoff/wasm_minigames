# Physics

Time to interact with this world. Bear in mind that, once again, the world only exists in GPU-land. It's never a bunch of polygons.

So there are a bunch of options:
1. Implement a CPU function that runs the same code as the GPU. This would involve rewriting the GPU code in rust, and as it samples textures this would also involve bundling the png library. If we want to sample lots of points this could be quite slow.

2. Extract data from the GPU. Use the same GPU code as renders the world to also calculate collisions. This would be very simple to code, but extracting data from the GPU is really slow and in some previous experiments I've done, capped the framerate at like 10FPS on firefox on linux, which is pretty poor.

3. Write a GPU physics engine and never extract data to the CPU. If we represent an entity as a pixel in a texture, we can easily compute physics on it, update the texture etc. We could then feed that into our rendering code as the entities transform. This means we can simulate lots of objects, and not have any GPU/CPU bottlenecks. However it does load up our GPU even more (and there is already a lot of load on it from the deffered pipeline), and it is then an interesting challenge to write any gameplay logic as they also have to be shaders. 

Number 3 sounds like a lot of work. Number 2 is definitely poor, so number 1 it is I guess.

<canvas id="pilot/pilot_physics"></canvas>