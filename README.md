# Introduction

Minecraft-like game with infinite maps rendered using ray-tracing and path-tracing techniques.

This project is originally an assignment for our **introduction to OpenGL** course, nothing serious.

The project is entirely written in Rust and GLSL (for OpenGL).


## Samples

Images are more talkative than long texts, so here are few samples of what our engines are capable of:

<img src="/data/samples/sky_sctr.jpg" width="250" height="250"> <img src="/data/samples/lights.jpg" width="250" height="250">
<img src="/data/samples/hole.jpg" width="250" height="250"> <img src="/data/samples/trees.jpg" width="250" height="250">
<img src="/data/samples/water_deformation.jpg" width="250" height="250"> <img src="/data/samples/plants.jpg" width="250" height="250">

Here is a demonstration video that shows some scenes captured from the game (*you have to click on the image below*):
[![Demonstration Video](https://img.youtube.com/vi/cHuE7GmoEc8/0.jpg)](https://www.youtube.com/watch?v=cHuE7GmoEc8 "Demonstration Video")


**For further details about the conception of this project, you can watch the french (but subtitled) explanation video available [https://youtu.be/dpVvFUy8lug](**here**).**

## Warning

We made every resources used by the project exept for the Textures that we took from the internet, a **big thanks** to people who contributed to those **beautiful** and **free** textures! :heart:

# Functionalities

GlOPR is a simple Minecraft-like game, which use ray tracing and path tracing algorithms for rendering.

We implemented a map generator:
* generation of different biomes with a global coherence (big oceans, warm / cold / temperate zones, beaches ...)
* generation of columns with coherent size (using perlin noise) and smooth transition between biomes (ex: between plains and mountains)
* generation of decorations: flowers, cactus, various trees, grass, ...

We implemented a minimalistic game engine using AABB collisions.

We also implemented a raytracer:
* render blocks with high quality textures and normal mapping
* render total transparency (ex: for leaves and flowers)
* render shadows and reflection
* render refraction for water
* directed light for sun and ambient light for caves
* render flowers and grass with a wind animation

Also, we implemented a pathtracer as an alternative renderer for the game:
* same functionalities as the raytracer (except animations and water refraction)
* better handling of reflection and other light sources

Finally, we added a realistic skybox using Rayleigh diffusion.


# Performances
The player can enable/disable the path-tracing rendering, and adapt the resolution of the game to its needs.

The performances highly depends on the game resolution and on the place where the player is. In fact, in a enclosed space (a hole for example), the ray-casting algorithm we implemented intersects almost instantly with surrounding objects, while it has to explore a higher number of voxel in open sky spaces.

**We did not seriously benchmark the game**, and the following numbers are **just observations we made** during the development of the game.

### Ray-tracing only:

- **Intel UHD Graphics 620**:   5fps to  30fps
- **NVIDIA GeForce GTX 1050**: 10fps to  60fps
- **NVIDIA GeForce RTX 2080**: 10fps to 200fps

### Path-tracing enabled:

- **Intel UHD Graphics 620**:  0.5fps to 10fps
- **NVIDIA GeForce GTX 1050**: 2  fps to 20fps
- **NVIDIA GeForce RTX 2080**: 10 fps to 60fps

Those performances, if they are not astonishing, seem good to us.

# Compile me

1. Install rustup: https://rustup.rs/
2. Run: rustup install nightly
3. In this directory, run: rustup override set nightly
4. Run cargo build --release


# Usage

**Warning: the game can take a few minutes to launch the first time. ~Thanks~ our >1500-lines shader for that.**

Example:
```
cargo run --release -- game \
          --resolution-coeff 2 \
          --view-distance 10 \
          --layout fr
```

Main game parameters:
* resolution-coeff: if greater than 1, the game is run with a poorer quality
* view-distance: number of chunks seen in each direction
* layout: fr or us, main keyboard mapping
* world: world path to load
* flat: if presents, the map is flat
* seed: (number) world random seed; by default 0

# In game options

**Move** Z,Q,S,D (fr) or W,A,S,D (us)

**Break a block** Left click

**Place a block** Right click

**Toggle pathtracing** P

**Toggle ambient light** L

**Set sun position** K

**Do daylight cycle** N

**Sneak** Left-shift (the player will be slower but won't fall)

**Toggle sprint** Left-control (the player will be faster)

**Toggle fly mode** Double click on space


# Conclusion

Although the project went further than expected and hence took longer to implement, the achieved results really satisfied us. There still are some issues to fix, and the game doesn't have enough functionalities to really be called a game, however, we believe that's enough for an assignment.
