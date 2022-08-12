# Architecture Planning

I'm still a novice rustacean and open-gl-er, so I'm not going to
specify everything exactly, but we can make some pretty important
decisions already.

Once again, we're not making a game engine here, we're making a single
specific game, so we don't need to handle things generically.

## Resource Management
One issue in Swoop is that every sprite, despite being a quad, has a
different mesh. It would have required a bunch of refactoring to
allow them to share mesh data. It was the same for textures.

So let's have a global resources data store. Perhaps something like:

```rust

Textures {
    rock_texture: WebGlTexture,
    ship_texture: WebGlTexture,
    ...
}


struct Resources {
    textures: Textures,
    meshes: Meshes,
    shader_programs: ShaderPrograms
}
```

Why all this static-ness. Why not hashmaps? Well, we could, but then
we would lose rust's ability to typecheck. An opinion that I am slowly
coming to is that users gradually find ways to dodge rusts static type
checks by using indices into vectors, hash maps etc.

We can probably generate this resources struct from a folder of actual 
resources, so we don't need to code it by hand.

## Rendering

In the STL demo we did a forward rendering pipeline. For this game I'm
going to do a deferred renderer. The geometry will be rendered out to
a bunch of buffers. These buffers can then be combined in another shader
to output the final image.

Why did I chose deferred? Well, I'd like there to be cool effects like
fog, volumetric lighting, GI etc. These can all be approximated easily
using data from the signed distance field of the cave, but to do so,
every mesh needs to be able to sample the cave signed distance field.
It's easier to do so in a single deferred pass that has all the needed
data than it is to do it in each and every fragment shader.

For a basic deferred pipeline we need a geometry buffer that contains
data on:

1. Surface Normals (2xfloats)
2. Surface Depth (1x float)
3. Surface Color (3x float)
4. Surface Material Properties (3x float)

We can stuff this into two 32 bit float textures as I think 16 bit floats
should be fine.


# That's all???
Is this all the planning I'm going to do? Eh, probably. I'm not yet sure
the best way to handle entities, so hopefully this project will help
clear that up.
