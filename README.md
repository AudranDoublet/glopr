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

# Compile me

1. Install rustup: https://rustup.rs/
2. Run: rustup install nightly
3. In this directory, run: rustup override set nightly
4. Run cargo build --release

# Usage

Warning: the game can take a few minutes to launch the first time. Thanks our >1500-lines shader for that.

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
