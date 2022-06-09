# flappy-brid-rs

flappy-brid implementation using [Bevy](https://github.com/bevyengine/bevy)

## Run locally

```bash
cargo run --features bevy/dynamic
```

## Build for wasm

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/flappy-brid.wasm
```

## Serve on web

install serve from npm

```bash
npm i serve
```

run serve

```bash
npx serve
```

and it should serve on `http://localhost:3000`
