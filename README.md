# Side Effects

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