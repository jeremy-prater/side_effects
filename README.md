# Side Effects

## Game idea

A pack of [Tanukis](https://en.wikipedia.org/wiki/Japanese_raccoon_dog) need to survive during the winter by eating mushrooms

The mushrooms will have side effects on the individual Tanuki that eats it. This could be a positive or negative side effect.

Mushrooms are the only food source.

Predators will also exist which can harm the Tanuki

## Game design

This will a 3D game with models created in blender

Terrain could possibly be generated using a combination of high and low frequency [Perlin Noise](https://en.wikipedia.org/wiki/Perlin_noise)

This would also allow for collision detection based on a low-cost noise function and different levels based on the initial seed.

## Mushroom effects

We should have a discussion about permanent vs temporary effects 

### Positive

- Health boost
- Extra action slot for one turn

### Negative

- Death
- Lose a turn
- Lose an action for one turn

## Running in the browser

### Setup
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```

### Running locally

```
cargo run --target wasm32-unknown-unknown
```

### Creating a deployment

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
```