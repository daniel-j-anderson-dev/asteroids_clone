# Asteroids Clone

## Controls
- UP: move forward
- LEFT: rotate counter clockwise
- RIGHT: rotate clockwise

## Example
<img src="example.gif" width="50%" height="50%">

## Building and Running

Prerequisites
  - [Install rust](https://www.rust-lang.org/tools/install)

Platforms
  - Linux, MacOS, Windows
    - Build: `cargo build`. output will be in `./target/debug/asteroids`
    - Run: `cargo run`. this will build and immediately execute
  - wasm (Web Assembly)
    - Build:
      - `cargo run --example build_wasm`

      **OR**

      1) Add wasm target. `rustup target add wasm32-unknown-unknown`
      2) Build wasm binary. `cargo build --target wasm32-unknown-unknown`. output will be at `./target/wasm32-unknown-unknown/debug/asteroids.wasm`
      3) Copy the wasm binary to `./examples/wasm/asteroids.wasm`
    - Run:
      1) Build
      2) Run the example server `cargo run --example wasm`
      3) Open a [compatible browser](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility) to `localhost:5000`

